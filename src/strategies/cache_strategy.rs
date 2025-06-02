use std::sync::Arc;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use ethers::types::{Address, U256};
use tokio::sync::Mutex;
use anyhow::{Result, anyhow};
use tokio::time;
use tracing::{info, warn, error, debug};
use crate::cache::BorrowerCache;

// ヘルスファクターの変化率カテゴリ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChangeRateCategory {
    // 急激な変化（10%以上/時間）
    Rapid,
    // 中程度の変化（1-10%/時間）
    Moderate,
    // 緩やかな変化（1%未満/時間）
    Slow,
    // 安定（ほぼ変化なし）
    Stable,
}

// キャッシュ戦略の設定
#[derive(Debug, Clone)]
pub struct CacheStrategyConfig {
    // Redisサーバー接続URL
    pub redis_url: String,
    // キャッシュのキープレフィックス
    pub cache_prefix: String,
    // TTL（キャッシュ有効期間）設定
    pub ttl_settings: HashMap<ChangeRateCategory, u64>,
    // 変化率しきい値設定
    pub change_rate_thresholds: ChangRateThresholds,
    // インクリメンタルスキャン間隔（秒）
    pub incremental_scan_interval: u64,
    // フルスキャン間隔（秒）
    pub full_scan_interval: u64,
}

// 変化率のしきい値設定
#[derive(Debug, Clone)]
pub struct ChangRateThresholds {
    // 急激な変化と判定するしきい値（時間あたりの変化率%）
    pub rapid: f64,
    // 中程度の変化と判定するしきい値
    pub moderate: f64,
    // 緩やかな変化と判定するしきい値
    pub slow: f64,
}

impl Default for ChangRateThresholds {
    fn default() -> Self {
        Self {
            rapid: 10.0,    // 10%/時間以上
            moderate: 1.0,  // 1-10%/時間
            slow: 0.1,      // 0.1-1%/時間
                            // 0.1%未満はStable
        }
    }
}

// ヘルスファクターキャッシュ戦略
#[derive(Debug)]
pub struct HealthFactorCacheStrategy {
    // Redisキャッシュクライアント
    cache: Arc<Mutex<BorrowerCache>>,
    // キャッシュされたヘルスファクター
    // (borrower -> (health_factor, timestamp, category))
    health_factors: Arc<Mutex<HashMap<Address, (U256, Instant, ChangeRateCategory)>>>,
    // 最後のヘルスファクター（変化率計算用）
    // (borrower -> (previous_health_factor, timestamp))
    previous_health_factors: Arc<Mutex<HashMap<Address, (U256, Instant)>>>,
    // 最後のフルスキャン時刻
    last_full_scan: Arc<Mutex<Instant>>,
    // 最後のインクリメンタルスキャン時刻
    last_incremental_scan: Arc<Mutex<Instant>>,
    // 設定
    config: CacheStrategyConfig,
}

impl HealthFactorCacheStrategy {
    // 新しいキャッシュ戦略を作成
    pub async fn new(config: CacheStrategyConfig) -> Result<Self> {
        // Redisキャッシュを初期化
        let cache = BorrowerCache::new(&config.redis_url, &config.cache_prefix).await?;
        
        // 初期化
        let strategy = Self {
            cache: Arc::new(Mutex::new(cache)),
            health_factors: Arc::new(Mutex::new(HashMap::new())),
            previous_health_factors: Arc::new(Mutex::new(HashMap::new())),
            last_full_scan: Arc::new(Mutex::new(Instant::now())),
            last_incremental_scan: Arc::new(Mutex::new(Instant::now())),
            config,
        };
        
        // キャッシュ初期化
        {
            let mut cache = strategy.cache.lock().await;
            if let Err(e) = cache.init().await {
                warn!("キャッシュの初期化に失敗しました: {}", e);
            }
        }
        
        Ok(strategy)
    }
    
    // TTLを取得
    fn get_ttl_for_category(&self, category: ChangeRateCategory) -> Duration {
        let ttl_secs = match self.config.ttl_settings.get(&category) {
            Some(ttl) => *ttl,
            None => {
                // デフォルト値
                match category {
                    ChangeRateCategory::Rapid => 30,        // 30秒
                    ChangeRateCategory::Moderate => 120,    // 2分
                    ChangeRateCategory::Slow => 600,        // 10分
                    ChangeRateCategory::Stable => 1800,     // 30分
                }
            }
        };
        
        Duration::from_secs(ttl_secs)
    }
    
    // 変化率に基づいてカテゴリを判定
    fn categorize_change_rate(&self, change_rate_per_hour: f64) -> ChangeRateCategory {
        let thresholds = &self.config.change_rate_thresholds;
        
        if change_rate_per_hour >= thresholds.rapid {
            ChangeRateCategory::Rapid
        } else if change_rate_per_hour >= thresholds.moderate {
            ChangeRateCategory::Moderate
        } else if change_rate_per_hour >= thresholds.slow {
            ChangeRateCategory::Slow
        } else {
            ChangeRateCategory::Stable
        }
    }
    
    // ヘルスファクターの変化率を計算（時間あたりの%）
    fn calculate_change_rate(&self, current: U256, previous: U256, time_diff: Duration) -> f64 {
        if previous.is_zero() {
            return 0.0;
        }
        
        // U256の大きな値を扱うため、f64に変換して計算
        let current_f = current.as_u128() as f64;
        let previous_f = previous.as_u128() as f64;
        
        // 変化率を計算 (|current - previous| / previous)
        let abs_diff = if current > previous {
            current_f - previous_f
        } else {
            previous_f - current_f
        };
        
        let percent_change = (abs_diff / previous_f) * 100.0;
        
        // 時間あたりの変化率に正規化（1時間あたり）
        let hours = time_diff.as_secs_f64() / 3600.0;
        if hours > 0.0 {
            percent_change / hours
        } else {
            0.0 // 時間差がゼロに近い場合
        }
    }
    
    // 借り手のヘルスファクターを更新
    pub async fn update_health_factor(&self, borrower: Address, health_factor: U256) -> Result<ChangeRateCategory> {
        let now = Instant::now();
        
        // 前回の値を取得して変化率を計算
        let category = {
            let mut prev_hfs = self.previous_health_factors.lock().await;
            
            let category = if let Some((prev_hf, prev_time)) = prev_hfs.get(&borrower) {
                let time_diff = now.duration_since(*prev_time);
                
                // 変化率を計算
                let change_rate = self.calculate_change_rate(health_factor, *prev_hf, time_diff);
                
                // カテゴリを判定
                self.categorize_change_rate(change_rate)
            } else {
                // 初回は中程度の変化と仮定
                ChangeRateCategory::Moderate
            };
            
            // 今回の値を記録
            prev_hfs.insert(borrower, (health_factor, now));
            
            category
        };
        
        // キャッシュ更新
        {
            let mut cache = self.cache.lock().await;
            cache.cache_health_factor(borrower, health_factor).await?;
            
            // ローカルキャッシュも更新
            let mut hfs = self.health_factors.lock().await;
            hfs.insert(borrower, (health_factor, now, category));
        }
        
        debug!("借り手 {:?} のヘルスファクター更新: HF={}, カテゴリ={:?}", 
              borrower, health_factor, category);
        
        Ok(category)
    }
    
    // 複数の借り手のヘルスファクターを一括更新
    pub async fn update_health_factors_batch(&self, updates: &[(Address, U256)]) -> Result<HashMap<ChangeRateCategory, usize>> {
        let now = Instant::now();
        let mut categories = HashMap::new();
        
        // 各更新を処理
        let mut cache_batch = Vec::with_capacity(updates.len());
        
        // まず変化率を計算
        let category_updates = {
            let mut prev_hfs = self.previous_health_factors.lock().await;
            let mut result = Vec::with_capacity(updates.len());
            
            for (borrower, health_factor) in updates {
                let category = if let Some((prev_hf, prev_time)) = prev_hfs.get(borrower) {
                    let time_diff = now.duration_since(*prev_time);
                    let change_rate = self.calculate_change_rate(*health_factor, *prev_hf, time_diff);
                    self.categorize_change_rate(change_rate)
                } else {
                    ChangeRateCategory::Moderate
                };
                
                // 前回値を更新
                prev_hfs.insert(*borrower, (*health_factor, now));
                
                // カテゴリをカウント
                *categories.entry(category).or_insert(0) += 1;
                
                // カテゴリと共に結果に追加
                result.push((*borrower, *health_factor, category));
                
                // キャッシュバッチに追加
                cache_batch.push((*borrower, *health_factor));
            }
            
            result
        };
        
        // キャッシュを一括更新
        {
            let mut cache = self.cache.lock().await;
            cache.cache_health_factors_batch(&cache_batch).await?;
            
            // ローカルキャッシュも更新
            let mut hfs = self.health_factors.lock().await;
            for (borrower, health_factor, category) in category_updates {
                hfs.insert(borrower, (health_factor, now, category));
            }
        }
        
        info!("ヘルスファクター一括更新: {}件 - カテゴリ分布: {:?}", updates.len(), categories);
        
        Ok(categories)
    }
    
    // キャッシュからヘルスファクターを取得
    pub async fn get_health_factor(&self, borrower: &Address) -> Result<Option<U256>> {
        // まずローカルキャッシュをチェック
        {
            let hfs = self.health_factors.lock().await;
            if let Some((hf, _, _)) = hfs.get(borrower) {
                return Ok(Some(*hf));
            }
        }
        
        // Redisキャッシュから取得
        let mut cache = self.cache.lock().await;
        let hf = cache.get_health_factor(borrower).await?;
        
        Ok(hf)
    }
    
    // 複数の借り手のヘルスファクターをバッチ取得
    pub async fn get_health_factors_batch(&self, borrowers: &[Address]) -> Result<HashMap<Address, Option<U256>>> {
        if borrowers.is_empty() {
            return Ok(HashMap::new());
        }
        
        let mut result = HashMap::with_capacity(borrowers.len());
        let mut missing = Vec::new();
        
        // まずローカルキャッシュをチェック
        {
            let hfs = self.health_factors.lock().await;
            for borrower in borrowers {
                if let Some((hf, _, _)) = hfs.get(borrower) {
                    result.insert(*borrower, Some(*hf));
                } else {
                    missing.push(*borrower);
                }
            }
        }
        
        // 不足分をRedisから取得
        if !missing.is_empty() {
            let mut cache = self.cache.lock().await;
            let missing_results = cache.get_health_factors_batch(&missing).await?;
            
            for (borrower, hf) in missing_results {
                result.insert(borrower, hf);
            }
        }
        
        Ok(result)
    }
    
    // キャッシュを基にスキャンが必要な借り手を決定
    pub async fn identify_borrowers_to_scan(&self, all_borrowers: &[Address]) -> Result<(Vec<Address>, HashMap<ChangeRateCategory, usize>)> {
        let now = Instant::now();
        
        // 各カテゴリの借り手数をカウント
        let mut category_counts = HashMap::new();
        for c in &[
            ChangeRateCategory::Rapid,
            ChangeRateCategory::Moderate,
            ChangeRateCategory::Slow,
            ChangeRateCategory::Stable,
        ] {
            category_counts.insert(*c, 0);
        }
        
        // スキャンする必要のある借り手
        let mut to_scan = Vec::new();
        
        // 現在のキャッシュ状態を取得
        let health_factors = self.health_factors.lock().await;
        
        for borrower in all_borrowers {
            let should_scan = if let Some((_, timestamp, category)) = health_factors.get(borrower) {
                // 経過時間をチェック
                let elapsed = now.duration_since(*timestamp);
                let ttl = self.get_ttl_for_category(*category);
                
                // カウンターを更新
                *category_counts.entry(*category).or_insert(0) += 1;
                
                // TTLを超えていたらスキャンが必要
                elapsed > ttl
            } else {
                // キャッシュにない場合はスキャンが必要
                *category_counts.entry(ChangeRateCategory::Moderate).or_insert(0) += 1;
                true
            };
            
            if should_scan {
                to_scan.push(*borrower);
            }
        }
        
        info!("借り手スキャン分析: 合計={}, スキャン対象={}, カテゴリ分布={:?}", 
             all_borrowers.len(), to_scan.len(), category_counts);
        
        Ok((to_scan, category_counts))
    }
    
    // インクリメンタルスキャンの実行判断
    pub async fn should_run_incremental_scan(&self) -> bool {
        let now = Instant::now();
        
        let last_scan = {
            let guard = self.last_incremental_scan.lock().await;
            *guard
        };
        
        let elapsed = now.duration_since(last_scan);
        let interval = Duration::from_secs(self.config.incremental_scan_interval);
        
        elapsed >= interval
    }
    
    // フルスキャンの実行判断
    pub async fn should_run_full_scan(&self) -> bool {
        let now = Instant::now();
        
        let last_scan = {
            let guard = self.last_full_scan.lock().await;
            *guard
        };
        
        let elapsed = now.duration_since(last_scan);
        let interval = Duration::from_secs(self.config.full_scan_interval);
        
        elapsed >= interval
    }
    
    // インクリメンタルスキャンの実行記録
    pub async fn mark_incremental_scan_completed(&self) {
        let mut last_scan = self.last_incremental_scan.lock().await;
        *last_scan = Instant::now();
    }
    
    // フルスキャンの実行記録
    pub async fn mark_full_scan_completed(&self) {
        let mut last_scan = self.last_full_scan.lock().await;
        *last_scan = Instant::now();
    }
    
    // キャッシュの統計情報を取得
    pub async fn get_cache_stats(&self) -> Result<(usize, usize, HashMap<ChangeRateCategory, usize>)> {
        // Redisキャッシュ統計
        let redis_stats = {
            let mut cache = self.cache.lock().await;
            cache.get_stats().await?
        };
        
        // ローカルキャッシュ統計
        let (local_count, categories) = {
            let hfs = self.health_factors.lock().await;
            
            // カテゴリごとの数をカウント
            let mut category_counts = HashMap::new();
            for (_, _, category) in hfs.values() {
                *category_counts.entry(*category).or_insert(0) += 1;
            }
            
            (hfs.len(), category_counts)
        };
        
        Ok((redis_stats.0, local_count, categories))
    }
    
    // キャッシュメンテナンス実行
    pub async fn run_maintenance_loop(&self) -> Result<()> {
        info!("キャッシュメンテナンスループを開始します");
        
        let cleanup_interval = Duration::from_secs(60 * 5); // 5分ごと
        
        loop {
            // キャッシュをクリーンアップ
            let mut cache = self.cache.lock().await;
            if let Err(e) = cache.clear_old_entries().await {
                warn!("キャッシュクリーンアップ中にエラー: {}", e);
            }
            drop(cache);
            
            // カテゴリの統計情報を収集
            match self.get_cache_stats().await {
                Ok((size, hits, categories)) => {
                    info!("キャッシュ統計 - サイズ: {}, ヒット: {}, カテゴリ: {:?}", size, hits, categories);
                }
                Err(e) => {
                    warn!("キャッシュ統計収集中にエラー: {}", e);
                }
            }
            
            // 次のクリーンアップまで待機
            time::sleep(cleanup_interval).await;
        }
    }
    
    // 特定のヘルスファクター範囲の借り手のみを選択
    pub async fn filter_by_health_factor_range(&self, borrowers: &[Address], min_hf: Option<U256>, max_hf: Option<U256>) -> Result<Vec<Address>> {
        if min_hf.is_none() && max_hf.is_none() {
            return Ok(borrowers.to_vec());
        }
        
        let health_factors = self.get_health_factors_batch(borrowers).await?;
        let mut filtered = Vec::new();
        
        for borrower in borrowers {
            if let Some(Some(hf)) = health_factors.get(borrower) {
                let mut include = true;
                
                if let Some(min) = min_hf {
                    if *hf < min {
                        include = false;
                    }
                }
                
                if let Some(max) = max_hf {
                    if *hf > max {
                        include = false;
                    }
                }
                
                if include {
                    filtered.push(*borrower);
                }
            }
        }
        
        Ok(filtered)
    }
    
    // 最も危険な借り手をN人取得
    pub async fn get_most_at_risk_borrowers(&self, borrowers: &[Address], count: usize) -> Result<Vec<(Address, U256)>> {
        if borrowers.is_empty() || count == 0 {
            return Ok(Vec::new());
        }
        
        // ヘルスファクターを取得
        let health_factors = self.get_health_factors_batch(borrowers).await?;
        
        // 有効なヘルスファクターを持つ借り手のみを選択
        let mut with_hf = Vec::new();
        for borrower in borrowers {
            if let Some(Some(hf)) = health_factors.get(borrower) {
                with_hf.push((*borrower, *hf));
            }
        }
        
        // ヘルスファクターでソート（昇順）
        with_hf.sort_by(|a, b| a.1.cmp(&b.1));
        
        // 最もリスクの高い（HFが低い）借り手を返す
        Ok(with_hf.into_iter().take(count).collect())
    }
    
    pub async fn cleanup_cache(&self) -> Result<()> {
        let mut cache = self.cache.lock().await;
        if let Err(e) = cache.clear_old_entries().await {
            warn!("キャッシュのクリーンアップ中にエラーが発生しました: {}", e);
        }
        Ok(())
    }
} 