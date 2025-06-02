use std::sync::Arc;
use std::time::{Duration, Instant};
use ethers::types::{Address, U256};
use anyhow::{Result, anyhow};
use tokio::sync::Mutex;
use tokio::time;
use tracing::{info, warn, error, debug};

use crate::strategies::priority_queue_manager::{PriorityQueueManager, PriorityQueueConfig, BorrowerEvent};
use crate::strategies::multi_level_scanner::{MultiLevelScanner, ScannerConfig, ScanLevel};
use crate::strategies::cache_strategy::{HealthFactorCacheStrategy, CacheStrategyConfig, ChangRateThresholds};

/// 3つのアプローチを統合した清算戦略
#[derive(Debug, Clone)]
pub struct IntegratedLiquidationStrategy {
    // 優先度キューマネージャー
    priority_queue: Arc<PriorityQueueManager>,
    // マルチレベルスキャナー
    scanner: Arc<MultiLevelScanner>,
    // キャッシュ戦略
    cache_strategy: Arc<HealthFactorCacheStrategy>,
    // 最終メンテナンス実行時刻
    last_maintenance: Arc<Mutex<Instant>>,
    // 設定
    config: IntegratedStrategyConfig,
}

/// 統合戦略の設定
#[derive(Debug, Clone)]
pub struct IntegratedStrategyConfig {
    // 共通Redis設定
    pub redis_url: String,
    pub cache_prefix: String,
    // メンテナンス間隔（秒）
    pub maintenance_interval: u64,
    // スキャンバッチサイズ
    pub scan_batch_size: usize,
    // 並列度設定
    pub parallelism: usize,
    // 最大同時処理数
    pub max_concurrent_batches: usize,
}

impl Default for IntegratedStrategyConfig {
    fn default() -> Self {
        Self {
            redis_url: "redis://127.0.0.1:6379".to_string(),
            cache_prefix: "hyperliquid:liquidator".to_string(),
            maintenance_interval: 300, // 5分
            scan_batch_size: 100,
            parallelism: 3,
            max_concurrent_batches: 4,
        }
    }
}

impl IntegratedLiquidationStrategy {
    /// 新しい統合戦略を作成
    pub async fn new(config: IntegratedStrategyConfig) -> Result<Self> {
        // 優先度キューマネージャーの設定
        let queue_config = PriorityQueueConfig {
            redis_url: config.redis_url.clone(),
            queue_key: format!("{}:priority_queue", config.cache_prefix),
            min_scan_interval: 5,   // 5秒
            max_scan_interval: 900, // 15分
            event_scan_health_threshold: 1.1,
            batch_size: config.scan_batch_size,
            health_factor_update_interval: 60, // 1分
        };
        
        // マルチレベルスキャナーの設定
        let scanner_config = ScannerConfig {
            max_batch_queue_size: 200,
            max_cache_age: 300, // 5分
            normal_parallelism: config.parallelism,
            critical_parallelism: config.parallelism + 1,
            max_concurrent_batches: config.max_concurrent_batches,
        };
        
        // キャッシュ戦略の設定
        let cache_config = CacheStrategyConfig {
            redis_url: config.redis_url.clone(),
            cache_prefix: config.cache_prefix.clone(),
            ttl_settings: std::collections::HashMap::new(), // デフォルト値を使用
            change_rate_thresholds: ChangRateThresholds::default(),
            incremental_scan_interval: 60,  // 1分
            full_scan_interval: 3600,       // 1時間
        };
        
        // 各コンポーネントを初期化
        let priority_queue = Arc::new(PriorityQueueManager::new(queue_config).await?);
        let scanner = Arc::new(MultiLevelScanner::new(scanner_config));
        let cache_strategy = Arc::new(HealthFactorCacheStrategy::new(cache_config).await?);
        
        Ok(Self {
            priority_queue,
            scanner,
            cache_strategy,
            last_maintenance: Arc::new(Mutex::new(Instant::now())),
            config,
        })
    }
    
    /// イベントを処理
    pub async fn process_event(&self, event: BorrowerEvent) -> Result<()> {
        // 優先度キューマネージャーでイベント処理
        self.priority_queue.process_event(event.clone()).await?;
        
        // イベントタイプに基づいてスキャンとキャッシュ更新
        match &event {
            BorrowerEvent::NewBorrow(borrower, _) | 
            BorrowerEvent::WithdrawCollateral(borrower, _) => {
                // リスクの高いイベントはCriticalレベルに設定
                self.scanner.add_borrower_to_level(*borrower, ScanLevel::Critical).await?;
            },
            BorrowerEvent::NewCollateral(borrower, _) => {
                // 担保追加はハイリスクとして処理
                self.scanner.add_borrower_to_level(*borrower, ScanLevel::HighRisk).await?;
            },
            BorrowerEvent::Repay(borrower, _) => {
                // 返済は中リスクに下げる
                self.scanner.add_borrower_to_level(*borrower, ScanLevel::MediumRisk).await?;
            },
            BorrowerEvent::Liquidation(borrower, _, _) => {
                // 清算済みはスキャナーから一時的に削除
                self.scanner.remove_borrower(*borrower).await?;
            },
            BorrowerEvent::HealthFactorUpdate(borrower, health_factor) => {
                // ヘルスファクター更新はすべてのコンポーネントに伝播
                let _level = ScanLevel::from_health_factor(*health_factor);
                
                // 健全性ファクターに基づいてキャッシュを更新
                if let Err(e) = self.cache_strategy.update_health_factor(*borrower, *health_factor).await {
                    warn!("ヘルスファクター更新エラー: {}", e);
                }
            },
        }
        
        Ok(())
    }
    
    /// 次にスキャンすべき借り手のバッチを取得
    pub async fn get_next_batch_to_scan(&self) -> Result<Vec<Address>> {
        // 優先度キューから高優先度の借り手を取得
        let high_priority_batch = self.priority_queue.get_next_borrowers_to_scan().await?;
        if !high_priority_batch.is_empty() {
            return Ok(high_priority_batch.iter().map(|(addr, _)| *addr).collect());
        }
        
        // 優先度キューが空ならマルチレベルスキャナーからバッチを取得
        if let Some((_level, borrowers)) = self.scanner.get_next_scan_batch().await? {
            return Ok(borrowers);
        }
        
        Ok(vec![])
    }
    
    /// 借り手のヘルスファクターを更新
    pub async fn update_borrower_health_factor(&self, borrower: Address, health_factor: U256) -> Result<()> {
        // すべてのコンポーネントで更新
        self.priority_queue.update_borrower_health_factor(borrower, health_factor).await?;
        self.scanner.update_borrower_health_factor(borrower, health_factor).await?;
        self.cache_strategy.update_health_factor(borrower, health_factor).await?;
        
        Ok(())
    }
    
    /// 最適なスキャンバッチのサイズを取得
    pub async fn get_optimal_batch_size(&self, level: ScanLevel) -> usize {
        level.batch_size()
    }
    
    /// 最適な並列度を取得
    pub async fn get_optimal_parallelism(&self, level: ScanLevel) -> usize {
        level.parallelism()
    }
    
    /// メンテナンスタスクを実行
    pub async fn run_maintenance(&self) -> Result<()> {
        // 前回のメンテナンスから指定時間が経過しているか確認
        let mut last_maintenance = self.last_maintenance.lock().await;
        let now = Instant::now();
        let elapsed = now.duration_since(*last_maintenance);
        
        if elapsed < Duration::from_secs(self.config.maintenance_interval) {
            // メンテナンス間隔に達していない場合はスキップ
            return Ok(());
        }
        
        debug!("システムメンテナンスを実行します");
        
        // 各コンポーネントのメンテナンスを実行
        self.priority_queue.perform_maintenance().await?;
        self.cache_strategy.cleanup_cache().await?;
        
        // 最終メンテナンス時刻を更新
        *last_maintenance = now;
        
        Ok(())
    }
    
    /// キャッシュからヘルスファクターを取得（キャッシュミスの場合はnull）
    pub async fn get_cached_health_factor(&self, borrower: &Address) -> Result<Option<U256>> {
        self.cache_strategy.get_health_factor(borrower).await
    }
    
    /// 複数の借り手のヘルスファクターをバッチで取得
    pub async fn get_health_factors_batch(&self, borrowers: &[Address]) -> Result<Vec<(Address, Option<U256>)>> {
        let results = self.cache_strategy.get_health_factors_batch(borrowers).await?;
        
        // 結果をベクターに変換
        let mut health_factors = Vec::with_capacity(borrowers.len());
        for borrower in borrowers {
            let hf = results.get(borrower).cloned().unwrap_or(None);
            health_factors.push((*borrower, hf));
        }
        
        Ok(health_factors)
    }
    
    /// 各コンポーネントの統計情報を取得
    pub async fn get_stats(&self) -> Result<IntegratedStats> {
        // 優先度キューの統計
        let (queue_size, high_priority, low_priority) = self.priority_queue.get_queue_stats().await?;
        
        // マルチレベルスキャナーの統計
        let scanner_stats = self.scanner.get_stats().await;
        
        // キャッシュ戦略の統計
        let (cache_size, cache_hits, category_counts) = self.cache_strategy.get_cache_stats().await?;
        
        Ok(IntegratedStats {
            queue_size,
            high_priority,
            low_priority,
            scanner_stats,
            cache_size,
            cache_hits,
            category_counts,
        })
    }
}

/// 統合戦略の統計情報
#[derive(Debug)]
pub struct IntegratedStats {
    // 優先度キューの統計
    pub queue_size: usize,
    pub high_priority: usize,
    pub low_priority: usize,
    // スキャナーの統計
    pub scanner_stats: std::collections::HashMap<ScanLevel, usize>,
    // キャッシュの統計
    pub cache_size: usize,
    pub cache_hits: usize,
    pub category_counts: std::collections::HashMap<crate::strategies::cache_strategy::ChangeRateCategory, usize>,
} 