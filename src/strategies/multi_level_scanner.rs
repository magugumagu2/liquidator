use std::sync::Arc;
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use ethers::types::{Address, U256};
use tokio::sync::Mutex;
use anyhow::{Result, anyhow};
use futures::future::join_all;
use tracing::{info, warn, error, debug};
use tokio::time;

// スキャンレベルの列挙型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScanLevel {
    Critical,    // HF < 1.0
    HighRisk,    // 1.0 <= HF < 1.05
    MediumRisk,  // 1.05 <= HF < 1.2
    LowRisk,     // 1.2 <= HF < 1.5
    Safe,        // HF >= 1.5
}

impl ScanLevel {
    // ヘルスファクターからスキャンレベルを決定
    pub fn from_health_factor(hf: U256) -> Self {
        let one_eth = U256::from_dec_str("1000000000000000000").unwrap(); // 1.0 = 10^18
        
        if hf < one_eth {
            ScanLevel::Critical
        } else if hf < one_eth + one_eth / 20 { // 1.05
            ScanLevel::HighRisk
        } else if hf < one_eth + one_eth / 5 { // 1.2
            ScanLevel::MediumRisk
        } else if hf < one_eth + one_eth / 2 { // 1.5
            ScanLevel::LowRisk
        } else {
            ScanLevel::Safe
        }
    }
    
    // スキャン間隔を取得（秒）
    pub fn scan_interval(&self) -> u64 {
        match self {
            ScanLevel::Critical => 5,      // 5秒
            ScanLevel::HighRisk => 15,     // 15秒
            ScanLevel::MediumRisk => 60,   // 1分
            ScanLevel::LowRisk => 300,     // 5分
            ScanLevel::Safe => 900,        // 15分
        }
    }
    
    // バッチサイズを取得
    pub fn batch_size(&self) -> usize {
        match self {
            ScanLevel::Critical => 50,     // クリティカルレベルは小さいバッチで素早く処理
            ScanLevel::HighRisk => 50,
            ScanLevel::MediumRisk => 100,  // 中程度のリスクは標準的なバッチサイズ
            ScanLevel::LowRisk => 200,     // 低リスクは大きなバッチで効率化
            ScanLevel::Safe => 500,        // 安全な借り手は非常に大きなバッチでスキャン
        }
    }
    
    // 並列度を取得
    pub fn parallelism(&self) -> usize {
        match self {
            ScanLevel::Critical => 3,      // 並列度3（元は4だが調整済み）
            ScanLevel::HighRisk => 2,      // 並列度2（元は3だが調整済み）
            ScanLevel::MediumRisk => 1,    // 並列度1（元は2だが調整済み）
            ScanLevel::LowRisk => 1,
            ScanLevel::Safe => 1,
        }
    }
}

// バッチスキャンの結果
#[derive(Debug, Clone)]
pub struct BatchScanResult {
    pub level: ScanLevel,
    pub borrowers: Vec<Address>,
    pub health_factors: Vec<U256>,
    pub underwater_count: usize,
    pub duration: Duration,
}

// マルチレベルスキャナー
#[derive(Debug)]
pub struct MultiLevelScanner {
    // レベルごとの借り手マップ
    levels: Arc<Mutex<HashMap<ScanLevel, Vec<Address>>>>,
    // レベルごとの最終スキャン時刻
    last_scans: Arc<Mutex<HashMap<ScanLevel, Instant>>>,
    // 借り手のヘルスファクターキャッシュ
    health_factors: Arc<Mutex<HashMap<Address, (U256, Instant)>>>,
    // スキャンの設定
    config: ScannerConfig,
    // 保留中のバッチスキャン
    pending_batches: Arc<Mutex<VecDeque<(ScanLevel, Vec<Address>)>>>,
}

// スキャナーの設定
#[derive(Debug, Clone)]
pub struct ScannerConfig {
    // バッチキューの最大サイズ
    pub max_batch_queue_size: usize,
    // ヘルスファクターキャッシュの最大保持時間（秒）
    pub max_cache_age: u64,
    // 通常スキャン時の並列度
    pub normal_parallelism: usize,
    // クリティカルスキャン時の並列度
    pub critical_parallelism: usize,
    // 最大同時バッチスキャン数
    pub max_concurrent_batches: usize,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            max_batch_queue_size: 100,
            max_cache_age: 300, // 5分
            normal_parallelism: 2,
            critical_parallelism: 4,
            max_concurrent_batches: 3,
        }
    }
}

impl MultiLevelScanner {
    // 新しいスキャナーを作成
    pub fn new(config: ScannerConfig) -> Self {
        Self {
            levels: Arc::new(Mutex::new(HashMap::new())),
            last_scans: Arc::new(Mutex::new(HashMap::new())),
            health_factors: Arc::new(Mutex::new(HashMap::new())),
            config,
            pending_batches: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
    
    // 借り手をスキャナーに追加
    pub async fn add_borrower(&self, borrower: Address, health_factor: Option<U256>) -> Result<()> {
        // ヘルスファクターが指定されていれば、それを使用
        if let Some(hf) = health_factor {
            let level = ScanLevel::from_health_factor(hf);
            self.add_borrower_to_level(borrower, level).await?;
            
            // キャッシュに追加
            let mut cache = self.health_factors.lock().await;
            cache.insert(borrower, (hf, Instant::now()));
        } else {
            // ヘルスファクターが不明の場合は、安全のためHighRiskとして扱う
            self.add_borrower_to_level(borrower, ScanLevel::HighRisk).await?;
        }
        
        Ok(())
    }
    
    // 特定のレベルに借り手を追加
    pub async fn add_borrower_to_level(&self, borrower: Address, level: ScanLevel) -> Result<()> {
        let mut levels = self.levels.lock().await;
        
        // 既存のレベルからは削除
        for (_, borrowers) in levels.iter_mut() {
            borrowers.retain(|b| *b != borrower);
        }
        
        // 新しいレベルに追加
        levels.entry(level)
            .or_insert_with(Vec::new)
            .push(borrower);
        
        debug!("借り手 {:?} をレベル {:?} に追加しました", borrower, level);
        Ok(())
    }
    
    // 借り手のヘルスファクターを更新
    pub async fn update_borrower_health_factor(&self, borrower: Address, health_factor: U256) -> Result<()> {
        let level = ScanLevel::from_health_factor(health_factor);
        self.add_borrower_to_level(borrower, level).await?;
        
        // キャッシュも更新
        let mut cache = self.health_factors.lock().await;
        cache.insert(borrower, (health_factor, Instant::now()));
        
        Ok(())
    }
    
    // 借り手をスキャナーから削除
    pub async fn remove_borrower(&self, borrower: Address) -> Result<()> {
        let mut levels = self.levels.lock().await;
        
        // すべてのレベルから削除
        for (_, borrowers) in levels.iter_mut() {
            borrowers.retain(|b| *b != borrower);
        }
        
        // キャッシュからも削除
        let mut cache = self.health_factors.lock().await;
        cache.remove(&borrower);
        
        debug!("借り手 {:?} をスキャナーから削除しました", borrower);
        Ok(())
    }
    
    // 次のスキャンバッチを取得
    pub async fn get_next_scan_batch(&self) -> Result<Option<(ScanLevel, Vec<Address>)>> {
        // 先にキューから取得を試みる
        let mut pending = self.pending_batches.lock().await;
        if let Some(batch) = pending.pop_front() {
            return Ok(Some(batch));
        }
        
        // キューが空ければ、新しいバッチを生成
        let batch = self.prepare_next_batch().await?;
        Ok(batch)
    }
    
    // 次のバッチを準備
    async fn prepare_next_batch(&self) -> Result<Option<(ScanLevel, Vec<Address>)>> {
        let levels = self.levels.lock().await;
        let mut last_scans = self.last_scans.lock().await;
        
        let now = Instant::now();
        let mut best_level = None;
        let mut best_priority = 0.0;
        
        // 各レベルの優先度を計算して最適なレベルを選択
        for (level, borrowers) in levels.iter() {
            if borrowers.is_empty() {
                continue;
            }
            
            // 前回のスキャンからの経過時間を取得
            let last_scan = last_scans.get(level).copied().unwrap_or_else(|| Instant::now() - Duration::from_secs(level.scan_interval() * 2));
            let elapsed = now.duration_since(last_scan);
            
            // レベルの優先度を計算：経過時間 / スキャン間隔
            let interval = Duration::from_secs(level.scan_interval());
            let priority = elapsed.as_secs_f64() / interval.as_secs_f64();
            
            // Critical/HighRiskレベルは優先度ボーナスを追加
            let priority_bonus = match level {
                ScanLevel::Critical => 2.0,
                ScanLevel::HighRisk => 1.5,
                _ => 1.0,
            };
            
            let adjusted_priority = priority * priority_bonus;
            
            // より優先度の高いレベルを選択
            if adjusted_priority > best_priority {
                best_level = Some(*level);
                best_priority = adjusted_priority;
            }
        }
        
        // 最適なレベルが見つかった場合
        if let Some(level) = best_level {
            // 最終スキャン時刻を更新
            last_scans.insert(level, now);
            
            // バッチサイズを決定
            let batch_size = level.batch_size();
            
            // 借り手をバッチサイズ分取得
            let borrowers = levels.get(&level)
                .map(|all| all.iter().take(batch_size).copied().collect::<Vec<_>>())
                .unwrap_or_default();
            
            if !borrowers.is_empty() {
                debug!("レベル {:?} のバッチスキャン準備：借り手数={}", level, borrowers.len());
                return Ok(Some((level, borrowers)));
            }
        }
        
        // 適切なバッチが見つからなければNone
        Ok(None)
    }
    
    // バッチをキューに追加
    pub async fn queue_batch(&self, level: ScanLevel, borrowers: Vec<Address>) -> Result<()> {
        let mut pending = self.pending_batches.lock().await;
        
        // キューが一杯なら追加しない
        if pending.len() >= self.config.max_batch_queue_size {
            warn!("バッチキューが満杯です（サイズ：{}）", pending.len());
            return Ok(());
        }
        
        // バッチサイズが0なら追加しない
        if borrowers.is_empty() {
            return Ok(());
        }
        
        pending.push_back((level, borrowers.clone()));
        
        debug!("レベル {:?} のバッチをキューに追加：借り手数={}", level, borrowers.len());
        Ok(())
    }
    
    // 全レベルの統計情報を取得
    pub async fn get_stats(&self) -> HashMap<ScanLevel, usize> {
        let levels = self.levels.lock().await;
        let mut stats = HashMap::new();
        
        for (level, borrowers) in levels.iter() {
            stats.insert(*level, borrowers.len());
        }
        
        stats
    }
    
    // バッチスキャンを実行
    pub async fn run_batch_scan<F, Fut>(&self, batch: (ScanLevel, Vec<Address>), scan_fn: F) -> Result<BatchScanResult>
    where
        F: Fn(Address) -> Fut + Send + Sync + Clone + 'static,
        Fut: std::future::Future<Output = Result<Option<U256>>> + Send + 'static,
    {
        let (level, borrowers) = batch;
        let start_time = Instant::now();
        
        info!("レベル {:?} のバッチスキャン開始：借り手数={}", level, borrowers.len());
        
        // 並列度の決定
        let parallelism = level.parallelism();
        
        // タスクへの分割
        let chunks = chunk_borrowers(&borrowers, parallelism);
        let mut tasks = Vec::with_capacity(chunks.len());
        
        // 各タスクを生成
        for chunk in chunks {
            let scan_fn = scan_fn.clone();
            let task = tokio::spawn(async move {
                let mut results = Vec::with_capacity(chunk.len());
                
                for borrower in chunk {
                    match scan_fn(borrower).await {
                        Ok(Some(hf)) => results.push((borrower, Some(hf))),
                        Ok(None) => results.push((borrower, None)),
                        Err(e) => {
                            error!("借り手 {:?} のスキャンエラー: {}", borrower, e);
                            results.push((borrower, None));
                        }
                    }
                }
                
                results
            });
            
            tasks.push(task);
        }
        
        // すべてのタスクを並列実行
        let mut all_results = Vec::with_capacity(borrowers.len());
        for task_result in join_all(tasks).await {
            match task_result {
                Ok(results) => all_results.extend(results),
                Err(e) => error!("バッチスキャンタスクエラー: {}", e),
            }
        }
        
        // 結果を処理
        let mut health_factors = Vec::new();
        let mut updated_borrowers = Vec::new();
        let mut underwater_count = 0;
        
        let one_eth = U256::from_dec_str("1000000000000000000").unwrap();
        
        let mut cache = self.health_factors.lock().await;
        
        for (borrower, health_factor) in all_results {
            if let Some(hf) = health_factor {
                // ヘルスファクターがあれば保存
                health_factors.push(hf);
                updated_borrowers.push(borrower);
                
                // キャッシュを更新
                cache.insert(borrower, (hf, Instant::now()));
                
                // 水没状態をチェック
                if hf < one_eth {
                    underwater_count += 1;
                }
                
                // レベルを更新
                let new_level = ScanLevel::from_health_factor(hf);
                if new_level != level {
                    self.add_borrower_to_level(borrower, new_level).await?;
                }
            }
        }
        
        let duration = start_time.elapsed();
        
        info!("レベル {:?} のバッチスキャン完了：処理数={}, 水没={}, 所要時間={:?}", 
             level, updated_borrowers.len(), underwater_count, duration);
        
        Ok(BatchScanResult {
            level,
            borrowers: updated_borrowers,
            health_factors,
            underwater_count,
            duration,
        })
    }
    
    // 古いキャッシュをクリーンアップ
    pub async fn cleanup_cache(&self) -> Result<usize> {
        let mut cache = self.health_factors.lock().await;
        let now = Instant::now();
        let max_age = Duration::from_secs(self.config.max_cache_age);
        
        let before_count = cache.len();
        
        // 古いエントリを削除
        cache.retain(|_, (_, timestamp)| {
            now.duration_since(*timestamp) < max_age
        });
        
        let removed = before_count - cache.len();
        
        if removed > 0 {
            debug!("ヘルスファクターキャッシュから{}エントリを削除しました", removed);
        }
        
        Ok(removed)
    }
    
    // 借り手のバッチ処理ワーカーを実行
    pub async fn run_batch_worker<F, Fut>(&self, scan_fn: F) -> Result<()>
    where
        F: Fn(Address) -> Fut + Send + Sync + Clone + 'static,
        Fut: std::future::Future<Output = Result<Option<U256>>> + Send + 'static,
    {
        let concurrent_count = Arc::new(Mutex::new(0usize));
        
        loop {
            // バッチを取得
            let batch_opt = {
                let next_batch = self.get_next_scan_batch().await?;
                if next_batch.is_none() {
                    // キューが空なら短時間スリープ
                    time::sleep(Duration::from_millis(100)).await;
                }
                next_batch
            };
            
            if let Some(batch) = batch_opt {
                // 実行中のバッチ数をチェック
                {
                    let count = concurrent_count.lock().await;
                    if *count >= self.config.max_concurrent_batches {
                        // 同時実行上限に達している場合は一時停止
                        time::sleep(Duration::from_millis(100)).await;
                        continue;
                    }
                }
                
                // 同時実行カウントを増加
                {
                    let mut count = concurrent_count.lock().await;
                    *count += 1;
                }
                
                // スキャン関数のクローンとカウンターの参照
                let scan_fn_clone = scan_fn.clone();
                let counter_ref = Arc::clone(&concurrent_count);
                
                // バッチを分解
                let (_level, borrowers) = batch;
                
                // バッチを非同期処理
                tokio::spawn(async move {
                    // 終了時にカウンターを減少させるガード
                    struct BorrowersRunner {
                        counter: Arc<Mutex<usize>>,
                    }
                    
                    impl Drop for BorrowersRunner {
                        fn drop(&mut self) {
                            let counter_clone = self.counter.clone();
                            tokio::spawn(async move {
                                let mut count = counter_clone.lock().await;
                                *count = count.saturating_sub(1);
                            });
                        }
                    }
                    
                    // ガードを作成
                    let _guard = BorrowersRunner { counter: counter_ref };
                    
                    // 各借り手に対してスキャン関数を実行
                    for borrower in borrowers {
                        if let Err(e) = scan_fn_clone(borrower).await {
                            error!("バッチスキャン中にエラーが発生しました: {}", e);
                        }
                    }
                });
            }
        }
    }
}

// 借り手のリストを並列タスク用にチャンクに分割
fn chunk_borrowers(borrowers: &[Address], parallelism: usize) -> Vec<Vec<Address>> {
    if borrowers.is_empty() || parallelism == 0 {
        return vec![];
    }
    
    let chunk_size = (borrowers.len() + parallelism - 1) / parallelism;
    let mut chunks = Vec::with_capacity(parallelism);
    
    for chunk_borrowers in borrowers.chunks(chunk_size) {
        chunks.push(chunk_borrowers.to_vec());
    }
    
    chunks
} 