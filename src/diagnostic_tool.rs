use crate::metrics::DiagnosticMetrics;
use crate::diagnostics::ArtemisChannelMonitor;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::time;
use tokio::sync::mpsc;
use tokio::task;
use tracing::{error, info, warn, debug};
use structopt::StructOpt;
use anyhow::{Result, anyhow};
use std::fs::File;
use std::io::Write;
use indicatif::{ProgressBar, ProgressStyle};
use ethers::providers::{Http, Provider};
use serde_json::Value;
use chrono;

#[derive(Debug, StructOpt)]
#[structopt(name = "liquidator-diagnostics", about = "チャネルラグ問題の診断ツール")]
pub struct DiagnosticOpts {
    /// ログファイルのパス
    #[structopt(short, long, parse(from_os_str))]
    log_file: PathBuf,
    
    /// 診断結果の出力ファイル
    #[structopt(short, long, parse(from_os_str))]
    output_file: Option<PathBuf>,
    
    /// 診断の実行時間（秒）
    #[structopt(short, long, default_value = "3600")]
    duration: u64,
    
    /// レポート間隔（秒）
    #[structopt(short, long, default_value = "300")]
    report_interval: u64,
}

pub async fn run_diagnostics(opts: DiagnosticOpts) -> Result<()> {
    // 診断メトリクスの初期化
    let metrics = Arc::new(DiagnosticMetrics::new());
    
    // ログモニターの初期化
    let monitor = Arc::new(ArtemisChannelMonitor::new(metrics.clone()));
    
    info!("診断ツールを開始しました");
    info!("ログファイル: {:?}", opts.log_file);
    info!("診断時間: {}秒", opts.duration);
    
    // 初期ログスキャン
    let detected = monitor.scan_log_file(&opts.log_file).await?;
    info!("初期スキャンで{}件のチャネルラグイベントを検出しました", detected);
    
    if detected > 0 {
        // 初期分析の実行
        let analysis = monitor.analyze_lag_pattern().await;
        info!("初期分析結果:\n{}", analysis);
        
        // システムリソース確認
        let resources = monitor.check_system_resources().await;
        info!("システムリソース状況:\n{}", resources);
    }
    
    // タイマー設定
    let report_interval = Duration::from_secs(opts.report_interval);
    let end_time = time::Instant::now() + Duration::from_secs(opts.duration);
    
    // 定期的なログスキャンとレポート生成
    loop {
        // 終了時間をチェック
        if time::Instant::now() >= end_time {
            info!("設定された診断時間 {}秒 が経過しました。診断を終了します。", opts.duration);
            break;
        }
        
        // 一定時間待機
        time::sleep(report_interval).await;
        
        // ログファイルの再スキャン
        let newly_detected = monitor.scan_log_file(&opts.log_file).await?;
        info!("追加で{}件のチャネルラグイベントを検出しました", newly_detected);
        
        // 定期レポート生成
        if newly_detected > 0 {
            // ラグパターン分析
            let analysis = monitor.analyze_lag_pattern().await;
            info!("ラグパターン分析結果:\n{}", analysis);
            
            // システムリソース確認
            let resources = monitor.check_system_resources().await;
            info!("システムリソース状況:\n{}", resources);
            
            // 総合メトリクスレポート
            metrics.maybe_report(true).await;
        } else {
            // イベントがない場合は簡易レポートのみ
            metrics.maybe_report(false).await;
        }
    }
    
    // 最終レポート生成
    info!("診断終了。最終レポートを生成します。");
    
    let final_analysis = monitor.analyze_lag_pattern().await;
    let final_resources = monitor.check_system_resources().await;
    let final_metrics = metrics.generate_report().await;
    
    let final_report = format!(
        "=== 清算ボット診断最終レポート ===\n\n{}\n\n{}\n\n{}\n",
        final_analysis, final_resources, final_metrics
    );
    
    info!("最終診断レポート:\n{}", final_report);
    
    // 出力ファイルへの書き込み
    if let Some(output_path) = opts.output_file {
        match std::fs::write(&output_path, final_report) {
            Ok(_) => info!("診断結果を {:?} に保存しました", output_path),
            Err(e) => error!("診断結果の保存に失敗しました: {}", e),
        }
    }
    
    Ok(())
}

// チャネルバッファサイズの検査機能
pub async fn inspect_channel_buffer() -> Result<()> {
    let metrics = Arc::new(DiagnosticMetrics::new());
    let now = SystemTime::now();
    let timestamp = now.duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
    let output_file = format!("channel_buffer_test_{}.csv", timestamp);
    
    info!("チャネルバッファサイズの検査を開始します");
    info!("結果は {} に保存されます", output_file);
    
    // 初期設定とCSVファイルの準備
    let mut file = File::create(&output_file)?;
    writeln!(file, "buffer_size,producer_rate,consumer_rate,messages_sent,messages_received,messages_dropped,duration_ms,lag_events")?;
    
    // テスト用のバッファサイズ一覧（10,000から5,000,000まで）
    let buffer_sizes = [10_000, 50_000, 100_000, 500_000, 1_000_000, 5_000_000];
    
    // 各バッファサイズに対してテストを実行
    for &buffer_size in &buffer_sizes {
        for producer_rate in [100, 500, 1000, 2000].iter() {
            for consumer_delay in [1, 5, 10, 20].iter() {
                info!("テスト: バッファサイズ={}, 生成レート={}個/秒, 消費遅延={}ms", 
                     buffer_size, producer_rate, consumer_delay);
                
                // 進捗バーの設定
                let pb = ProgressBar::new(100);
                pb.set_style(ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                    .unwrap()
                    .progress_chars("##-"));
                pb.set_message(format!("バッファサイズ: {}", buffer_size));

                // テスト実行
                let (messages_sent, messages_received, messages_dropped, duration_ms, lag_events) = 
                    test_channel_buffer(buffer_size, *producer_rate, *consumer_delay, &pb).await?;
                
                // 結果をCSVに記録
                writeln!(file, "{},{},{},{},{},{},{},{}",
                       buffer_size, producer_rate, consumer_delay, 
                       messages_sent, messages_received, messages_dropped, 
                       duration_ms, lag_events)?;
                
                pb.finish_with_message(format!(
                    "完了: 送信={}, 受信={}, 破棄={}, ラグ={}", 
                    messages_sent, messages_received, messages_dropped, lag_events
                ));
                
                // 少し休憩
                time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
    
    info!("チャネルバッファサイズのテストが完了しました");
    info!("結果は {} に保存されました", output_file);
    
    Ok(())
}

// チャネルバッファのテスト実行関数
async fn test_channel_buffer(
    buffer_size: usize, 
    producer_rate: u64,
    consumer_delay: u64,
    progress: &ProgressBar
) -> Result<(usize, usize, usize, u128, usize)> {
    // メトリクス
    let mut messages_sent = 0;
    let mut messages_received = 0;
    let mut messages_dropped = 0;
    let mut lag_events = 0;
    
    // 大きめのメッセージを用意（バッファのテストのため）
    let test_message = vec![0u8; 1024]; // 1KBのメッセージ
    
    // チャネルの作成
    let (tx, mut rx) = mpsc::channel(buffer_size);
    
    // テスト時間は10秒
    let test_duration = Duration::from_secs(10);
    let start = Instant::now();
    let end_time = start + test_duration;
    
    // 生成者タスクの開始
    let producer_handle = task::spawn(async move {
        let mut local_sent = 0;
        let mut local_dropped = 0;
        let interval = Duration::from_micros(1_000_000 / producer_rate);
        let mut next_send = Instant::now();
        
        while Instant::now() < end_time {
            // メッセージ送信
            if let Err(_) = tx.try_send(test_message.clone()) {
                local_dropped += 1;
                
                // チャネルラグ発生
                if local_dropped % 1000 == 0 {
                    debug!("チャネルラグが発生しています: {} メッセージが破棄されました", local_dropped);
                }
            } else {
                local_sent += 1;
            }
            
            // レート制御
            next_send += interval;
            if next_send > Instant::now() {
                time::sleep_until(next_send.into()).await;
            }
        }
        
        (local_sent, local_dropped)
    });
    
    // 消費者タスクの開始
    let consumer_handle = task::spawn(async move {
        let mut local_received = 0;
        let mut local_lag_events = 0;
        
        while let Some(_message) = rx.recv().await {
            local_received += 1;
            
            // 消費遅延のシミュレーション
            time::sleep(Duration::from_millis(consumer_delay)).await;
            
            // バッファが80%以上埋まったらラグイベントとしてカウント
            if rx.capacity() < buffer_size / 5 {
                local_lag_events += 1;
                if local_lag_events % 100 == 0 {
                    debug!("バッファが逼迫しています: 残り容量 {}/{}", rx.capacity(), buffer_size);
                }
            }
            
            if Instant::now() >= end_time {
                break;
            }
        }
        
        (local_received, local_lag_events)
    });
    
    // 進捗表示
    let update_interval = test_duration.as_millis() / 100;
    for i in 0..100 {
        progress.set_position(i + 1);
        time::sleep(Duration::from_millis(update_interval as u64 / 1000)).await;
    }
    
    // 結果収集
    let (producer_result, consumer_result) = tokio::join!(producer_handle, consumer_handle);
    
    match producer_result {
        Ok((sent, dropped)) => {
            messages_sent = sent;
            messages_dropped = dropped;
        },
        Err(e) => error!("生成者タスクエラー: {}", e),
    }
    
    match consumer_result {
        Ok((received, lags)) => {
            messages_received = received;
            lag_events = lags;
        },
        Err(e) => error!("消費者タスクエラー: {}", e),
    }
    
    let duration_ms = start.elapsed().as_millis();
    
    Ok((messages_sent, messages_received, messages_dropped, duration_ms, lag_events))
}

// 処理タスク数の検査機能
pub async fn inspect_task_count() -> Result<()> {
    let metrics = Arc::new(DiagnosticMetrics::new());
    
    // タスク生成と完了をモニタリング
    for i in 0..10 {
        let task_count = metrics.task_started();
        info!("テストタスク {} 開始 (アクティブタスク: {})", i, task_count);
        
        // 模擬的な処理時間
        time::sleep(Duration::from_millis(100)).await;
        
        let task_count = metrics.task_completed();
        info!("テストタスク {} 完了 (アクティブタスク: {})", i, task_count);
    }
    
    Ok(())
}

// RPC応答時間の検査機能
pub async fn inspect_rpc_performance() -> Result<()> {
    let metrics = Arc::new(DiagnosticMetrics::new());
    
    info!("RPC応答時間テストを開始します");
    
    // テスト用のRPC呼び出しを模擬
    for i in 0..5 {
        metrics.rpc_call_started();
        
        // 模擬的なRPC処理時間
        let delay = Duration::from_millis(100 * (i + 1));
        info!("テストRPC呼び出し {} ({}ms)", i, delay.as_millis());
        time::sleep(delay).await;
        
        metrics.rpc_call_completed();
        metrics.record_rpc_duration(delay).await;
    }
    
    // レポート出力
    let report = metrics.generate_report().await;
    info!("RPC応答時間テスト結果:\n{}", report);
    
    Ok(())
}

pub async fn inspect_json_response_size(rpc_url: &str) -> Result<()> {
    let provider = Provider::<Http>::try_from(rpc_url)?;
    
    // HyperLiquidのサポートされているRPCメソッドをテスト
    let test_cases = vec![
        ("net_version", vec![]),
        ("web3_clientVersion", vec![]),
        ("eth_chainId", vec![]),
        ("eth_blockNumber", vec![]),
        ("eth_gasPrice", vec![]),
        ("eth_getBalance", vec![
            "0x0000000000000000000000000000000000000000".to_string(),
            "latest".to_string()
        ]),
    ];
    
    info!("HyperLiquid RPCエンドポイントの診断を開始します");
    info!("URL: {}", rpc_url);
    
    for (method, params) in test_cases {
        let start = Instant::now();
        match provider.request::<Vec<String>, Value>(method, params).await {
            Ok(response) => {
                let elapsed = start.elapsed();
                
                // レスポンスサイズを計算
                let response_str = serde_json::to_string(&response)?;
                let size_bytes = response_str.len();
                let size_kb = size_bytes as f64 / 1024.0;
                
                info!("メソッド: {}", method);
                info!("レスポンスサイズ: {:.2} KB", size_kb);
                info!("応答時間: {:?}", elapsed);
                
                // 大きなレスポンスの警告
                if size_kb > 1000.0 {
                    warn!("警告: 大きなレスポンスサイズ ({:.2} KB)", size_kb);
                }
                
                // 遅い応答の警告
                if elapsed > Duration::from_secs(1) {
                    warn!("警告: 遅い応答時間 ({:?})", elapsed);
                }
            },
            Err(e) => {
                warn!("メソッド {} の実行中にエラーが発生しました: {}", method, e);
            }
        }
    }
    
    // 追加の診断情報
    info!("RPCエンドポイントの診断情報:");
    info!("テスト完了時刻: {}", chrono::Utc::now());
    
    Ok(())
} 