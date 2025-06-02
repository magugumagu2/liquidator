use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tracing::{info, warn};

#[derive(Debug)]
pub struct DiagnosticMetrics {
    // タスク関連のメトリクス
    active_tasks: AtomicUsize,
    max_tasks_observed: AtomicUsize,
    task_durations: Mutex<VecDeque<(String, Duration)>>,
    
    // チャネル関連のメトリクス
    channel_lags: Mutex<HashMap<String, Vec<usize>>>,
    
    // イベント処理関連のメトリクス
    events_generated: AtomicUsize,
    events_processed: AtomicUsize,
    
    // RPC関連のメトリクス
    pending_rpc_calls: AtomicUsize,
    max_pending_rpc_calls: AtomicUsize,
    rpc_call_durations: Mutex<VecDeque<Duration>>,
    
    // 時間計測
    last_report_time: Mutex<Instant>,
    creation_time: Instant,
}

impl Default for DiagnosticMetrics {
    fn default() -> Self {
        Self {
            active_tasks: AtomicUsize::new(0),
            max_tasks_observed: AtomicUsize::new(0),
            task_durations: Mutex::new(VecDeque::new()),
            channel_lags: Mutex::new(HashMap::new()),
            events_generated: AtomicUsize::new(0),
            events_processed: AtomicUsize::new(0),
            pending_rpc_calls: AtomicUsize::new(0),
            max_pending_rpc_calls: AtomicUsize::new(0),
            rpc_call_durations: Mutex::new(VecDeque::new()),
            last_report_time: Mutex::new(Instant::now()),
            creation_time: Instant::now(),
        }
    }
}

impl DiagnosticMetrics {
    pub fn new() -> Self {
        Self {
            last_report_time: Mutex::new(Instant::now()),
            creation_time: Instant::now(),
            ..Default::default()
        }
    }
    
    // タスク関連のメトリクス記録
    pub fn task_started(&self) -> usize {
        let current = self.active_tasks.fetch_add(1, Ordering::SeqCst) + 1;
        let max = self.max_tasks_observed.load(Ordering::SeqCst);
        if current > max {
            self.max_tasks_observed.store(current, Ordering::SeqCst);
        }
        current
    }
    
    pub fn task_completed(&self) -> usize {
        self.active_tasks.fetch_sub(1, Ordering::SeqCst) - 1
    }
    
    pub async fn record_task_duration(&self, task_name: String, duration: Duration) {
        let mut durations = self.task_durations.lock().await;
        durations.push_back((task_name, duration));
        // 最大100件保持
        if durations.len() > 100 {
            durations.pop_front();
        }
    }
    
    // チャネルラグの記録
    pub async fn record_channel_lag(&self, channel_name: &str, lag_value: usize) {
        let mut lags = self.channel_lags.lock().await;
        lags.entry(channel_name.to_string())
            .or_insert_with(Vec::new)
            .push(lag_value);
    }
    
    // イベント関連のメトリクス記録
    pub fn event_generated(&self) -> usize {
        self.events_generated.fetch_add(1, Ordering::SeqCst)
    }
    
    pub fn event_processed(&self) -> usize {
        self.events_processed.fetch_add(1, Ordering::SeqCst)
    }
    
    // RPC関連のメトリクス記録
    pub fn rpc_call_started(&self) -> usize {
        let current = self.pending_rpc_calls.fetch_add(1, Ordering::SeqCst) + 1;
        let max = self.max_pending_rpc_calls.load(Ordering::SeqCst);
        if current > max {
            self.max_pending_rpc_calls.store(current, Ordering::SeqCst);
        }
        current
    }
    
    pub fn rpc_call_completed(&self) -> usize {
        self.pending_rpc_calls.fetch_sub(1, Ordering::SeqCst) - 1
    }
    
    pub async fn record_rpc_duration(&self, duration: Duration) {
        let mut durations = self.rpc_call_durations.lock().await;
        durations.push_back(duration);
        // 最大100件保持
        if durations.len() > 100 {
            durations.pop_front();
        }
    }
    
    // レポート生成
    pub async fn generate_report(&self) -> String {
        let mut report = String::new();
        let uptime = self.creation_time.elapsed();
        
        // 基本情報
        report.push_str(&format!("=== 診断レポート (稼働時間: {:.2}分) ===\n", 
            uptime.as_secs_f64() / 60.0));
        
        // タスク情報
        report.push_str(&format!("\n== タスク情報 ==\n"));
        report.push_str(&format!("アクティブタスク数: {}\n", 
            self.active_tasks.load(Ordering::SeqCst)));
        report.push_str(&format!("観測された最大タスク数: {}\n", 
            self.max_tasks_observed.load(Ordering::SeqCst)));
        
        // タスク実行時間の統計
        let durations = self.task_durations.lock().await;
        if !durations.is_empty() {
            let mut total_ms = 0.0;
            let mut max_duration = Duration::from_millis(0);
            let mut max_task_name = String::new();
            
            for (task_name, duration) in durations.iter() {
                total_ms += duration.as_millis() as f64;
                if *duration > max_duration {
                    max_duration = *duration;
                    max_task_name = task_name.clone();
                }
            }
            
            let avg_ms = total_ms / durations.len() as f64;
            report.push_str(&format!("平均タスク実行時間: {:.2}ms\n", avg_ms));
            report.push_str(&format!("最長タスク: {} ({:.2}ms)\n", 
                max_task_name, max_duration.as_millis()));
        }
        
        // チャネルラグ情報
        report.push_str(&format!("\n== チャネルラグ情報 ==\n"));
        let lags = self.channel_lags.lock().await;
        for (channel, lag_values) in lags.iter() {
            if lag_values.is_empty() {
                continue;
            }
            
            let avg_lag = lag_values.iter().sum::<usize>() as f64 / lag_values.len() as f64;
            let max_lag = lag_values.iter().max().unwrap_or(&0);
            report.push_str(&format!("チャネル '{}': 平均ラグ: {:.1}, 最大ラグ: {}, 発生回数: {}\n", 
                channel, avg_lag, max_lag, lag_values.len()));
        }
        
        // イベント処理情報
        report.push_str(&format!("\n== イベント処理情報 ==\n"));
        let generated = self.events_generated.load(Ordering::SeqCst);
        let processed = self.events_processed.load(Ordering::SeqCst);
        report.push_str(&format!("生成されたイベント: {}\n", generated));
        report.push_str(&format!("処理されたイベント: {}\n", processed));
        
        if generated > 0 && processed > 0 {
            let ratio = generated as f64 / processed as f64;
            report.push_str(&format!("生成/処理比率: {:.2}\n", ratio));
            if ratio > 1.1 {
                report.push_str("警告: イベント生成速度が処理速度を上回っています\n");
            }
        }
        
        // RPC呼び出し情報
        report.push_str(&format!("\n== RPC呼び出し情報 ==\n"));
        report.push_str(&format!("保留中のRPC呼び出し: {}\n", 
            self.pending_rpc_calls.load(Ordering::SeqCst)));
        report.push_str(&format!("観測された最大保留RPC呼び出し: {}\n", 
            self.max_pending_rpc_calls.load(Ordering::SeqCst)));
        
        // RPC応答時間の統計
        let rpc_durations = self.rpc_call_durations.lock().await;
        if !rpc_durations.is_empty() {
            let total_ms = rpc_durations.iter()
                .map(|d| d.as_millis() as f64)
                .sum::<f64>();
            let avg_ms = total_ms / rpc_durations.len() as f64;
            let max_ms = rpc_durations.iter()
                .map(|d| d.as_millis())
                .max()
                .unwrap_or(0);
            
            report.push_str(&format!("平均RPC応答時間: {:.2}ms\n", avg_ms));
            report.push_str(&format!("最大RPC応答時間: {}ms\n", max_ms));
            
            if avg_ms > 1000.0 {
                report.push_str("警告: RPC応答時間が1秒を超えています\n");
            }
        }
        
        report.push_str("\n===========================\n");
        report
    }
    
    // 定期的なレポート出力（必要に応じて呼び出す）
    pub async fn maybe_report(&self, force: bool) {
        let mut last_report = self.last_report_time.lock().await;
        if force || last_report.elapsed() > Duration::from_secs(300) { // 5分ごと
            let report = self.generate_report().await;
            info!("診断レポート:\n{}", report);
            *last_report = Instant::now();
        }
    }
    
    // チャネル監視ユーティリティ
    pub fn monitor_channel_capacity<T>(
        &self, 
        sender: &tokio::sync::mpsc::Sender<T>, 
        channel_name: &str
    ) -> f64 {
        let capacity = sender.capacity();
        let max_capacity = sender.max_capacity();
        let usage_percent = 100.0 * (max_capacity - capacity) as f64 / max_capacity as f64;
        
        if usage_percent > 80.0 {
            warn!("チャネル'{}' バッファが容量に近づいています: {:.2}%", 
                 channel_name, usage_percent);
        }
        
        usage_percent
    }
}

// タスク実行時間測定のユーティリティラッパー
pub async fn measure_task_time<F, Fut, T>(
    metrics: Option<&Arc<DiagnosticMetrics>>,
    task_name: &str,
    f: F
) -> T
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = T>,
{
    if let Some(metrics) = metrics {
        metrics.task_started();
    }
    
    let start = Instant::now();
    let result = f().await;
    let elapsed = start.elapsed();
    
    if let Some(metrics) = metrics {
        metrics.task_completed();
        metrics.record_task_duration(task_name.to_string(), elapsed).await;
    }
    
    // 長時間実行タスクの警告
    if elapsed > Duration::from_millis(500) {
        warn!("タスク '{}' の実行に時間がかかりました: {}ms", task_name, elapsed.as_millis());
    }
    
    result
}

// RPC呼び出しのメトリクス記録ラッパー
pub async fn measure_rpc_call<F, Fut, T>(
    metrics: Option<&Arc<DiagnosticMetrics>>,
    rpc_name: &str,
    f: F
) -> Result<T, Box<dyn std::error::Error>>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T, Box<dyn std::error::Error>>>,
{
    if let Some(metrics) = metrics {
        metrics.rpc_call_started();
    }
    
    let start = Instant::now();
    let result = f().await;
    let elapsed = start.elapsed();
    
    if let Some(metrics) = metrics {
        metrics.rpc_call_completed();
        metrics.record_rpc_duration(elapsed).await;
    }
    
    // 長時間RPC呼び出しの警告
    if elapsed > Duration::from_millis(1000) {
        warn!("RPC呼び出し '{}' の応答に時間がかかりました: {}ms", rpc_name, elapsed.as_millis());
    }
    
    result
}

// チャネルラグをキャプチャするためのパッチ
pub fn patch_artemis_engine() -> anyhow::Result<()> {
    // 注: これは実際の実装ではなく疑似コードです
    // Artemisエンジンへのモンキーパッチは、このモジュールの外部で実装する必要があります
    warn!("Artemisエンジンへのパッチ適用は、このモジュールでは直接実装できません");
    Ok(())
} 