use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use ethers::types::{Address, U256};
use anyhow::{Result, anyhow};
use tokio::sync::Mutex;
use tokio::time;
use crate::priority_queue::PriorityQueue;
use tracing::{info, warn, error, debug};

// 借り手のスキャン状態を表す列挙型
#[derive(Debug, Clone, PartialEq)]
pub enum BorrowerScanState {
    // 初期状態
    New,
    // スキャン中（最終スキャン時刻）
    Scanning(Instant),
    // スキャン完了（ヘルスファクターと最終スキャン時刻）
    Scanned(U256, Instant),
    // イベントによる更新（イベント種別と時刻）
    EventTriggered(String, Instant),
}

// イベント種別を表す列挙型
#[derive(Debug, Clone, PartialEq)]
pub enum BorrowerEvent {
    // 新しい借入が発生
    NewBorrow(Address, Address), // (borrower, asset)
    // 新しい担保が追加
    NewCollateral(Address, Address), // (borrower, asset)
    // 返済が発生
    Repay(Address, Address), // (borrower, asset)
    // 担保が引き出された
    WithdrawCollateral(Address, Address), // (borrower, asset)
    // 清算が発生
    Liquidation(Address, Address, Address), // (borrower, collateral, debt)
    // 外部からヘルスファクター更新が通知された
    HealthFactorUpdate(Address, U256), // (borrower, new health factor)
}

// 優先度キューマネージャー
#[derive(Debug)]
pub struct PriorityQueueManager {
    // Redis優先度キュー
    queue: Arc<Mutex<PriorityQueue>>,
    // 借り手の状態マップ
    borrower_states: Arc<Mutex<std::collections::HashMap<Address, BorrowerScanState>>>,
    // 最後のイベント処理時刻
    last_event_processing: Arc<Mutex<Instant>>,
    // 設定
    config: PriorityQueueConfig,
}

// 優先度キューの設定
#[derive(Debug, Clone)]
pub struct PriorityQueueConfig {
    // RedisサーバーのURL
    pub redis_url: String,
    // キューのキー名
    pub queue_key: String,
    // 最小スキャン間隔（秒）
    pub min_scan_interval: u64,
    // 最大スキャン間隔（秒）
    pub max_scan_interval: u64,
    // イベント駆動スキャンの閾値ヘルスファクター
    pub event_scan_health_threshold: f64,
    // バッチサイズ
    pub batch_size: usize,
    // ヘルスファクターの更新間隔（秒）
    pub health_factor_update_interval: u64,
}

impl PriorityQueueManager {
    // 新しいマネージャーを作成
    pub async fn new(config: PriorityQueueConfig) -> Result<Self> {
        // Redis優先度キューを初期化
        let queue = PriorityQueue::new(&config.redis_url, &config.queue_key).await?;
        
        Ok(Self {
            queue: Arc::new(Mutex::new(queue)),
            borrower_states: Arc::new(Mutex::new(std::collections::HashMap::new())),
            last_event_processing: Arc::new(Mutex::new(Instant::now())),
            config,
        })
    }
    
    // イベントを処理してキューを更新
    pub async fn process_event(&self, event: BorrowerEvent) -> Result<()> {
        match event {
            BorrowerEvent::NewBorrow(borrower, _) => {
                // 新しい借入のイベント：優先度を高く設定
                self.add_high_priority_borrower(borrower).await?;
                self.update_borrower_state(borrower, BorrowerScanState::EventTriggered("new_borrow".to_string(), Instant::now())).await;
            },
            BorrowerEvent::NewCollateral(borrower, _) => {
                // 担保追加のイベント：通常の優先度で追加
                self.add_borrower_for_scan(borrower).await?;
                self.update_borrower_state(borrower, BorrowerScanState::EventTriggered("new_collateral".to_string(), Instant::now())).await;
            },
            BorrowerEvent::Repay(borrower, _) => {
                // 返済イベント：優先度を低く更新
                self.update_borrower_health_factor(borrower, U256::from(2) * U256::exp10(18)).await?;
                self.update_borrower_state(borrower, BorrowerScanState::EventTriggered("repay".to_string(), Instant::now())).await;
            },
            BorrowerEvent::WithdrawCollateral(borrower, _) => {
                // 担保引き出しイベント：優先度を高く設定
                self.add_high_priority_borrower(borrower).await?;
                self.update_borrower_state(borrower, BorrowerScanState::EventTriggered("withdraw_collateral".to_string(), Instant::now())).await;
            },
            BorrowerEvent::Liquidation(borrower, _, _) => {
                // 清算イベント：キューから一時的に削除
                self.remove_borrower(borrower).await?;
                self.update_borrower_state(borrower, BorrowerScanState::EventTriggered("liquidation".to_string(), Instant::now())).await;
            },
            BorrowerEvent::HealthFactorUpdate(borrower, health_factor) => {
                // ヘルスファクターの更新：優先度を更新
                self.update_borrower_health_factor(borrower, health_factor).await?;
                self.update_borrower_state(borrower, BorrowerScanState::Scanned(health_factor, Instant::now())).await;
            },
        }
        
        // 最後のイベント処理時刻を更新
        let mut last_processing = self.last_event_processing.lock().await;
        *last_processing = Instant::now();
        
        Ok(())
    }
    
    // 借り手の状態を更新
    async fn update_borrower_state(&self, borrower: Address, state: BorrowerScanState) {
        let mut states = self.borrower_states.lock().await;
        states.insert(borrower, state);
    }
    
    // 高優先度で借り手をキューに追加
    async fn add_high_priority_borrower(&self, borrower: Address) -> Result<()> {
        // ヘルスファクター1.01のような低い値で追加して高優先度にする
        let health_factor = U256::from(101) * U256::exp10(16); // 1.01 * 10^18
        let mut queue = self.queue.lock().await;
        queue.add_borrower(borrower, health_factor).await?;
        
        debug!("高優先度で借り手を追加しました: {:?}", borrower);
        Ok(())
    }
    
    // 通常優先度で借り手をキューに追加
    pub async fn add_borrower_for_scan(&self, borrower: Address) -> Result<()> {
        // 中間程度のヘルスファクター値（1.5）
        let health_factor = U256::from(15) * U256::exp10(17); // 1.5 * 10^18
        let mut queue = self.queue.lock().await;
        queue.add_borrower(borrower, health_factor).await?;
        
        let mut states = self.borrower_states.lock().await;
        if !states.contains_key(&borrower) {
            states.insert(borrower, BorrowerScanState::New);
        }
        
        debug!("スキャン対象に借り手を追加しました: {:?}", borrower);
        Ok(())
    }
    
    // ヘルスファクターに基づいて借り手の優先度を更新
    pub async fn update_borrower_health_factor(&self, borrower: Address, health_factor: U256) -> Result<()> {
        let mut queue = self.queue.lock().await;
        queue.add_borrower(borrower, health_factor).await?;
        
        debug!("借り手のヘルスファクターを更新しました: {:?}, HF: {}", borrower, health_factor);
        Ok(())
    }
    
    // 借り手をキューから削除
    pub async fn remove_borrower(&self, borrower: Address) -> Result<()> {
        let mut queue = self.queue.lock().await;
        queue.remove_borrower(borrower).await?;
        
        debug!("借り手をキューから削除しました: {:?}", borrower);
        Ok(())
    }
    
    // 次にスキャンすべき借り手のバッチを取得
    pub async fn get_next_borrowers_to_scan(&self) -> Result<Vec<(Address, f64)>> {
        let mut queue = self.queue.lock().await;
        let count = self.config.batch_size as isize;
        let borrowers = queue.get_highest_priority(count).await?;
        
        // スキャン状態を更新
        let mut states = self.borrower_states.lock().await;
        for (borrower, _) in &borrowers {
            states.insert(*borrower, BorrowerScanState::Scanning(Instant::now()));
        }
        
        Ok(borrowers)
    }
    
    // キューの統計情報を取得
    pub async fn get_queue_stats(&self) -> Result<(usize, usize, usize)> {
        let mut queue = self.queue.lock().await;
        
        // キューの合計サイズ
        let total_size = queue.size().await?;
        
        // 危険なヘルスファクター（<1.0）の数
        let underwater_count = queue.count_in_range(0.0, 1.0).await?;
        
        // 警告ヘルスファクター（1.0-1.2）の数
        let warning_count = queue.count_in_range(1.0, 1.2).await?;
        
        Ok((total_size, underwater_count, warning_count))
    }
    
    // 定期的なキュー更新とメンテナンスを実行
    pub async fn run_maintenance_loop(&self) -> Result<()> {
        let interval = Duration::from_secs(30); // 30秒ごとに実行
        
        loop {
            // メンテナンス処理
            if let Err(e) = self.perform_maintenance().await {
                error!("キューメンテナンス処理エラー: {}", e);
            }
            
            // 統計情報の出力
            if let Ok((total, underwater, warning)) = self.get_queue_stats().await {
                info!("キュー統計: 合計={}, 水没={}, 警告={}", total, underwater, warning);
            }
            
            // 一定時間待機
            time::sleep(interval).await;
        }
    }
    
    // キューのメンテナンス処理
    pub async fn perform_maintenance(&self) -> Result<()> {
        let mut states = self.borrower_states.lock().await;
        
        // 古いイベント状態をクリーンアップ
        let now = Instant::now();
        let old_threshold = Duration::from_secs(self.config.max_scan_interval * 2);
        
        // 削除対象の借り手リスト
        let mut to_clean = Vec::new();
        
        for (borrower, state) in states.iter() {
            match state {
                BorrowerScanState::EventTriggered(_, event_time) => {
                    let elapsed = now.duration_since(*event_time);
                    if elapsed > old_threshold {
                        to_clean.push(*borrower);
                    }
                },
                BorrowerScanState::Scanning(scan_time) => {
                    let elapsed = now.duration_since(*scan_time);
                    // 長時間スキャン中のままの場合（スキャンがフリーズした可能性）
                    if elapsed > Duration::from_secs(600) { // 10分以上スキャン中
                        to_clean.push(*borrower);
                    }
                },
                _ => {}
            }
        }
        
        // 古い状態を削除
        for borrower in to_clean {
            states.remove(&borrower);
            debug!("メンテナンス: 借り手の古い状態を削除 {:?}", borrower);
        }
        
        Ok(())
    }
    
    // イベント駆動型でスキャンすべき借り手を判断
    pub async fn should_scan_borrower_event_driven(&self, borrower: Address) -> Result<bool> {
        // イベント駆動型スキャンの条件：
        // 1. 重要なイベントが発生した場合
        // 2. ヘルスファクターが閾値を下回る場合
        // 3. 前回スキャンから最小間隔以上が経過した場合
        
        // イベント情報を確認
        let states = self.borrower_states.lock().await;
        if let Some(state) = states.get(&borrower) {
            match state {
                BorrowerScanState::EventTriggered(event_type, event_time) => {
                    // 最小スキャン間隔以上経過したか確認
                    let now = Instant::now();
                    let elapsed = now.duration_since(*event_time);
                    let min_interval = Duration::from_secs(self.config.min_scan_interval);
                    
                    if elapsed < min_interval {
                        // まだ最小間隔が経過していない
                        return Ok(false);
                    }
                    
                    // 重要なイベントタイプかどうか確認
                    match event_type.as_str() {
                        "new_borrow" | "withdraw_collateral" => Ok(true), // 高リスクイベント
                        _ => {
                            // その他のイベントはヘルスファクターを確認
                            match self.get_borrower_health_factor(borrower).await {
                                Ok(Some(hf)) => {
                                    // ヘルスファクターを浮動小数点に変換（1.0 = 10^18）
                                    let hf_float = hf as f64 / 1e18;
                                    
                                    Ok(hf_float < self.config.event_scan_health_threshold)
                                }
                                Ok(None) => {
                                    Ok(true) // ヘルスファクターが不明なら安全のためスキャン
                                }
                                Err(_) => {
                                    Ok(true) // 取得エラーがあれば安全のためスキャン
                                }
                            }
                        }
                    }
                },
                BorrowerScanState::Scanning(scan_time) | BorrowerScanState::Scanned(_, scan_time) => {
                    // 前回のスキャンからの経過時間をチェック
                    let now = Instant::now();
                    let elapsed = now.duration_since(*scan_time);
                    
                    // 最小スキャン間隔
                    let min_interval = Duration::from_secs(self.config.min_scan_interval);
                    
                    if elapsed >= min_interval {
                        // ヘルスファクターを確認し、リスクに基づいて動的にスキャン間隔を調整
                        match self.get_borrower_health_factor(borrower).await {
                            Ok(Some(hf)) => {
                                let hf_float = hf as f64 / 1e18;
                                // ヘルスファクターに基づくダイナミックスキャン間隔（1.0に近いほど頻繁に）
                                let dynamic_interval = (hf_float - 1.0).max(0.1) * self.config.max_scan_interval as f64;
                                
                                Ok(elapsed >= Duration::from_secs(dynamic_interval as u64))
                            }
                            _ => {
                                // ヘルスファクター不明なら最小間隔でスキャン
                                Ok(elapsed >= min_interval)
                            }
                        }
                    } else {
                        Ok(false) // 最小間隔未満ならスキャンしない
                    }
                },
                _ => Ok(true), // 新規状態など、不明な状態ならスキャン
            }
        } else {
            // 状態情報がないなら初期スキャン
            Ok(true)
        }
    }
    
    // 借り手の現在のヘルスファクターを取得
    async fn get_borrower_health_factor(&self, borrower: Address) -> Result<Option<u128>> {
        let mut queue = self.queue.lock().await;
        let score = queue.get_borrower_priority(borrower).await?;
        
        Ok(score.map(|s| (s * 1e18) as u128))
    }
} 