use crate::metrics::DiagnosticMetrics;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn};
use std::time::{Duration, Instant};
use regex::Regex;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::collections::VecDeque;

// チャネルラグの履歴
#[derive(Debug, Clone)]
pub struct ChannelLagEvent {
    timestamp: Instant,
    lag_value: usize,
    thread_id: Option<String>,
}

#[derive(Debug)]
pub struct ArtemisChannelMonitor {
    metrics: Arc<DiagnosticMetrics>,
    lag_events: Mutex<VecDeque<ChannelLagEvent>>,
    log_pattern: Regex,
    last_analysis: Mutex<Instant>,
}

impl ArtemisChannelMonitor {
    pub fn new(metrics: Arc<DiagnosticMetrics>) -> Self {
        Self {
            metrics,
            lag_events: Mutex::new(VecDeque::with_capacity(1000)),
            // エラーログのパターンを正規表現で定義
            log_pattern: Regex::new(r"ERROR artemis_core::engine: error receiving event: channel lagged by (\d+)").unwrap(),
            last_analysis: Mutex::new(Instant::now()),
        }
    }
    
    // ログファイルをパースしてチャネルラグイベントを検出
    pub async fn scan_log_file(&self, log_path: &Path) -> anyhow::Result<usize> {
        if !log_path.exists() {
            warn!("ログファイルが存在しません: {:?}", log_path);
            return Ok(0);
        }
        
        let file = File::open(log_path)?;
        let reader = BufReader::new(file);
        let mut detected_count = 0;
        
        for line in reader.lines() {
            if let Ok(line) = line {
                if let Some(captures) = self.log_pattern.captures(&line) {
                    if let Some(lag_str) = captures.get(1) {
                        if let Ok(lag_value) = lag_str.as_str().parse::<usize>() {
                            detected_count += 1;
                            self.record_lag_event(lag_value).await;
                        }
                    }
                }
            }
        }
        
        if detected_count > 0 {
            info!("ログファイルから{}件のチャネルラグイベントを検出しました", detected_count);
        }
        
        Ok(detected_count)
    }
    
    // リアルタイムロギングをキャプチャするためのモック関数
    // 実際には組み込みロガーを拡張して実装する必要があります
    pub async fn register_log_listener(&self) -> anyhow::Result<()> {
        warn!("ログリスナーの登録は現在の実装ではサポートされていません");
        Ok(())
    }
    
    // チャネルラグイベントの記録
    pub async fn record_lag_event(&self, lag_value: usize) -> usize {
        // メトリクスに記録
        self.metrics.record_channel_lag("artemis_engine", lag_value).await;
        
        // 内部履歴に追加
        let mut events = self.lag_events.lock().await;
        events.push_back(ChannelLagEvent {
            timestamp: Instant::now(),
            lag_value,
            thread_id: None,
        });
        
        // 最大1000件保持
        if events.len() > 1000 {
            events.pop_front();
        }
        
        events.len()
    }
    
    // チャネルラグの傾向分析
    pub async fn analyze_lag_pattern(&self) -> String {
        let mut last_analysis = self.last_analysis.lock().await;
        if last_analysis.elapsed() < Duration::from_secs(60) {
            // 分析は最低1分間隔で行う
            return "前回の分析から60秒経過していないため、分析をスキップします".to_string();
        }
        
        *last_analysis = Instant::now();
        let events = self.lag_events.lock().await;
        
        if events.is_empty() {
            return "分析対象のラグイベントがありません".to_string();
        }
        
        // 基本統計
        let total_events = events.len();
        let total_lag: usize = events.iter().map(|e| e.lag_value).sum();
        let avg_lag = total_lag as f64 / total_events as f64;
        let max_lag = events.iter().map(|e| e.lag_value).max().unwrap_or(0);
        
        // 時間帯分析（最近10分間を1分刻みで集計）
        let mut time_buckets = vec![0; 10];
        let now = Instant::now();
        
        for event in events.iter() {
            let elapsed = now.duration_since(event.timestamp);
            if elapsed <= Duration::from_secs(600) { // 10分以内
                let bucket = (elapsed.as_secs() / 60) as usize;
                if bucket < 10 {
                    time_buckets[bucket] += 1;
                }
            }
        }
        
        // 分析結果の文字列化
        let mut analysis = String::new();
        analysis.push_str(&format!("=== チャネルラグ分析 ===\n"));
        analysis.push_str(&format!("総イベント数: {}\n", total_events));
        analysis.push_str(&format!("平均ラグ値: {:.2}\n", avg_lag));
        analysis.push_str(&format!("最大ラグ値: {}\n", max_lag));
        
        analysis.push_str("\n== 時間帯別発生頻度 (直近10分) ==\n");
        for (i, count) in time_buckets.iter().enumerate() {
            analysis.push_str(&format!("{}分前: {}件\n", i, count));
        }
        
        // ラグ値の分布
        let mut lag_distribution = [0; 6]; // [<10, <100, <1000, <10000, <100000, >=100000]
        
        for event in events.iter() {
            let bucket = match event.lag_value {
                0..=9 => 0,
                10..=99 => 1,
                100..=999 => 2,
                1000..=9999 => 3,
                10000..=99999 => 4,
                _ => 5,
            };
            lag_distribution[bucket] += 1;
        }
        
        analysis.push_str("\n== ラグ値の分布 ==\n");
        let categories = ["<10", "10-99", "100-999", "1K-9.9K", "10K-99K", ">=100K"];
        for (i, count) in lag_distribution.iter().enumerate() {
            let percentage = (*count as f64 / total_events as f64) * 100.0;
            analysis.push_str(&format!("{}: {}件 ({:.1}%)\n", 
                                     categories[i], count, percentage));
        }
        
        // 問題の可能性の診断
        analysis.push_str("\n== 診断結果 ==\n");
        
        if avg_lag > 1000.0 {
            analysis.push_str("警告: 平均ラグ値が1000を超えています。これはイベント処理の深刻な遅延を示しています。\n");
        }
        
        if lag_distribution[3] + lag_distribution[4] + lag_distribution[5] > total_events / 10 {
            analysis.push_str("警告: 1000以上のラグ値が10%以上を占めています。システムの処理能力を超えている可能性があります。\n");
        }
        
        let recent_events = time_buckets[0] + time_buckets[1];
        if recent_events > 50 {
            analysis.push_str("警告: 直近2分間に50以上のラグイベントが発生しています。現在進行形の問題がある可能性があります。\n");
        }
        
        analysis.push_str("\n== 推奨対策 ==\n");
        
        if max_lag > 100000 {
            analysis.push_str("1. チャネルバッファサイズの大幅な増加を検討してください。\n");
        }
        
        if avg_lag > 100.0 {
            analysis.push_str("2. 並列タスクの数を減らしてリソース競合を軽減してください。\n");
        }
        
        let recent_trend = 
            if time_buckets[0] > time_buckets[1] && time_buckets[1] > time_buckets[2] {
                "増加"
            } else if time_buckets[0] < time_buckets[1] && time_buckets[1] < time_buckets[2] {
                "減少"
            } else {
                "変動"
            };
        
        analysis.push_str(&format!("3. ラグイベントの発生傾向: {}\n", recent_trend));
        
        if recent_trend == "増加" {
            analysis.push_str("   システムの負荷が増加している可能性があります。リソース使用状況を確認してください。\n");
        }
        
        analysis
    }
    
    // システムリソース使用状況の確認（Linuxシステム向け）
    pub async fn check_system_resources(&self) -> String {
        let mut result = String::new();
        result.push_str("=== システムリソース使用状況 ===\n");
        
        // CPUロード
        if let Ok(loads) = self.read_loadavg() {
            result.push_str(&format!("CPUロード平均: 1分:{:.2} 5分:{:.2} 15分:{:.2}\n", 
                                  loads.0, loads.1, loads.2));
        } else {
            result.push_str("CPUロード情報の取得に失敗しました\n");
        }
        
        // メモリ使用状況
        if let Ok(mem_info) = self.read_meminfo() {
            let used_percent = (mem_info.1 as f64 / mem_info.0 as f64) * 100.0;
            result.push_str(&format!("メモリ: 合計:{}MB 使用中:{}MB ({:.1}%)\n", 
                                  mem_info.0 / 1024, mem_info.1 / 1024, used_percent));
        } else {
            result.push_str("メモリ情報の取得に失敗しました\n");
        }
        
        // プロセス情報
        if let Ok(process_stats) = self.get_process_stats() {
            result.push_str(&format!("プロセスCPU使用率: {:.1}%\n", process_stats.0));
            result.push_str(&format!("プロセスメモリ使用量: {}MB\n", process_stats.1 / 1024));
            result.push_str(&format!("スレッド数: {}\n", process_stats.2));
        } else {
            result.push_str("プロセス情報の取得に失敗しました\n");
        }
        
        result
    }
    
    // /proc/loadavgからロードアベレージを読み取る
    fn read_loadavg(&self) -> anyhow::Result<(f64, f64, f64)> {
        let content = std::fs::read_to_string("/proc/loadavg")?;
        let parts: Vec<&str> = content.split_whitespace().collect();
        
        if parts.len() >= 3 {
            let load1 = parts[0].parse::<f64>()?;
            let load5 = parts[1].parse::<f64>()?;
            let load15 = parts[2].parse::<f64>()?;
            Ok((load1, load5, load15))
        } else {
            Err(anyhow::anyhow!("Invalid loadavg format"))
        }
    }
    
    // /proc/meminfoからメモリ情報を読み取る
    fn read_meminfo(&self) -> anyhow::Result<(usize, usize)> {
        let content = std::fs::read_to_string("/proc/meminfo")?;
        let mut total_kb = 0;
        let mut free_kb = 0;
        let mut buffers_kb = 0;
        let mut cached_kb = 0;
        
        for line in content.lines() {
            if line.starts_with("MemTotal:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    total_kb = parts[1].parse::<usize>()?;
                }
            } else if line.starts_with("MemFree:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    free_kb = parts[1].parse::<usize>()?;
                }
            } else if line.starts_with("Buffers:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    buffers_kb = parts[1].parse::<usize>()?;
                }
            } else if line.starts_with("Cached:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    cached_kb = parts[1].parse::<usize>()?;
                }
            }
        }
        
        let used_kb = total_kb - free_kb - buffers_kb - cached_kb;
        Ok((total_kb, used_kb))
    }
    
    // 自プロセスの統計情報を取得
    fn get_process_stats(&self) -> anyhow::Result<(f64, usize, usize)> {
        let pid = std::process::id();
        let stat_path = format!("/proc/{}/stat", pid);
        let status_path = format!("/proc/{}/status", pid);
        
        // CPU使用率を計算
        let content = std::fs::read_to_string(stat_path)?;
        let parts: Vec<&str> = content.split_whitespace().collect();
        
        if parts.len() < 15 {
            return Err(anyhow::anyhow!("Invalid process stat format"));
        }
        
        let utime = parts[13].parse::<usize>()?;
        let stime = parts[14].parse::<usize>()?;
        let total_time = utime + stime;
        
        // CPU使用率の計算は簡易的なものとして
        let cpu_usage = total_time as f64 / 100.0;
        
        // メモリ使用量とスレッド数
        let status_content = std::fs::read_to_string(status_path)?;
        let mut vm_rss = 0;
        let mut threads = 0;
        
        for line in status_content.lines() {
            if line.starts_with("VmRSS:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    vm_rss = parts[1].parse::<usize>()?;
                }
            } else if line.starts_with("Threads:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    threads = parts[1].parse::<usize>()?;
                }
            }
        }
        
        Ok((cpu_usage, vm_rss, threads))
    }
} 