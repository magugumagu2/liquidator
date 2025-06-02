use aave_v3_liquidator::metrics::DiagnosticMetrics;
use structopt::StructOpt;
use tokio::sync::mpsc;
use tokio::task;
use tokio::time::{self, Duration, Instant};
use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};
use std::fs::File;
use std::io::Write;
use std::time::SystemTime;
use tracing::{info, warn, debug, error, Level};
use tracing_subscriber::FmtSubscriber;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use csv::Writer;
use chrono;

#[derive(Debug, StructOpt)]
#[structopt(name = "channel-test", about = "チャネルバッファとラグのテストツール")]
pub struct Opts {
    /// 最小バッファサイズ
    #[structopt(long, default_value = "10000")]
    min_buffer: usize,

    /// 最大バッファサイズ
    #[structopt(long, default_value = "5000000")]
    max_buffer: usize,

    /// バッファサイズの増加係数
    #[structopt(long, default_value = "5")]
    buffer_multiplier: usize,

    /// 最小生成者スループット（メッセージ/秒）
    #[structopt(long, default_value = "100")]
    min_rate: u64,

    /// 最大生成者スループット（メッセージ/秒）
    #[structopt(long, default_value = "2000")]
    max_rate: u64,

    /// 消費者の平均処理時間（ミリ秒）
    #[structopt(long, default_value = "5")]
    consumer_delay: u64,

    /// 生成者並列数
    #[structopt(long, default_value = "3")]
    producers: usize,

    /// 消費者並列数
    #[structopt(long, default_value = "1")]
    consumers: usize,

    /// テスト時間（秒）
    #[structopt(long, default_value = "30")]
    duration: u64,

    /// 出力ファイル
    #[structopt(long)]
    output: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
struct TestResult {
    buffer_size: usize,
    producers: usize,
    consumers: usize,
    producer_rate: u64,
    consumer_delay: u64,
    messages_sent: usize,
    messages_received: usize,
    messages_dropped: usize,
    lag_events: usize,
    duration_ms: u128,
    effective_throughput: f64,
    drop_rate: f64,
}

#[tokio::main]
async fn main() -> Result<()> {
    // トレーシングの初期化
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("トレーシングの初期化に失敗しました");

    let opts = Opts::from_args();
    
    // 現在の時刻をタイムスタンプとして使用
    let now = chrono::Local::now();
    let timestamp = now.format("%Y%m%d%H%M%S").to_string();
    
    // 出力ファイル名を決定
    let output_file = opts.output.as_ref().map_or_else(
        || format!("channel_test_{}.csv", timestamp),
        |s| s.clone()
    );
    
    info!("チャネルバッファテストツールを開始します");
    info!("バッファサイズ: {} から {}", opts.min_buffer, opts.max_buffer);
    info!("生成者数: {}, 消費者数: {}", opts.producers, opts.consumers);
    info!("生成レート: {} から {} メッセージ/秒", opts.min_rate, opts.max_rate);
    info!("消費遅延: {}ms", opts.consumer_delay);
    info!("テスト時間: {}秒", opts.duration);
    info!("結果出力: {}", output_file);
    
    // CSVファイルの準備
    let mut wtr = Writer::from_path(&output_file)?;
    
    // テスト実行
    run_tests(
        &opts, 
        |result| {
            wtr.serialize(result).unwrap();
            wtr.flush().unwrap();
        }
    ).await?;
    
    info!("テスト完了。結果は {} に保存されました", output_file);
    
    Ok(())
}

async fn run_tests<F>(opts: &Opts, mut result_callback: F) -> Result<()>
where
    F: FnMut(TestResult),
{
    // テスト条件の組み合わせを生成
    let mut buffer_size = opts.min_buffer;
    let mut test_count = 0;
    
    // 全テスト数を計算
    let mut total_tests = 0;
    let mut buf = opts.min_buffer;
    while buf <= opts.max_buffer {
        total_tests += 4; // 4種類の生成レートをテスト
        buf *= opts.buffer_multiplier;
    }
    
    // 進捗バーの設定
    let pb = ProgressBar::new(total_tests);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("##-"));
    
    // バッファサイズを変えながらテスト
    while buffer_size <= opts.max_buffer {
        for producer_rate in [opts.min_rate, opts.min_rate * 5, opts.min_rate * 10, opts.max_rate].iter() {
            pb.set_message(format!("テスト {}/{}: バッファ={}, レート={}", 
                                 test_count + 1, total_tests, buffer_size, producer_rate));
            
            // テスト実行
            let result = test_channel_performance(
                buffer_size, 
                *producer_rate, 
                opts.consumer_delay,
                opts.producers,
                opts.consumers,
                opts.duration,
            ).await?;
            
            // 結果処理
            result_callback(result);
            
            test_count += 1;
            pb.inc(1);
        }
        
        // バッファサイズを増加
        buffer_size *= opts.buffer_multiplier;
    }
    
    pb.finish_with_message(format!("{}件のテストが完了しました", test_count));
    
    Ok(())
}

async fn test_channel_performance(
    buffer_size: usize,
    producer_rate: u64,
    consumer_delay: u64,
    producer_count: usize,
    consumer_count: usize,
    test_duration_secs: u64,
) -> Result<TestResult> {
    // 共有カウンター
    let messages_sent = Arc::new(AtomicUsize::new(0));
    let messages_received = Arc::new(AtomicUsize::new(0));
    let messages_dropped = Arc::new(AtomicUsize::new(0));
    let lag_events = Arc::new(AtomicUsize::new(0));
    
    // チャネルの作成
    let (tx, rx) = mpsc::channel(buffer_size);
    
    // テスト時間設定
    let test_duration = Duration::from_secs(test_duration_secs);
    let start = Instant::now();
    let end_time = start + test_duration;
    
    // 生成者タスクの開始
    let mut producer_handles = Vec::with_capacity(producer_count);
    
    for producer_id in 0..producer_count {
        let tx = tx.clone();
        let messages_sent = messages_sent.clone();
        let messages_dropped = messages_dropped.clone();
        let producer_rate_per_task = producer_rate / producer_count as u64;
        
        let handle = task::spawn(async move {
            // 大きめのメッセージを用意
            let test_message = vec![producer_id as u8; 1024]; // 1KBのメッセージ
            
            let interval = Duration::from_micros(1_000_000 / producer_rate_per_task);
            let mut next_send = Instant::now();
            
            while Instant::now() < end_time {
                // メッセージ送信
                if let Err(_) = tx.try_send(test_message.clone()) {
                    messages_dropped.fetch_add(1, Ordering::SeqCst);
                } else {
                    messages_sent.fetch_add(1, Ordering::SeqCst);
                }
                
                // レート制御
                next_send += interval;
                if next_send > Instant::now() {
                    time::sleep_until(next_send.into()).await;
                }
            }
        });
        
        producer_handles.push(handle);
    }
    
    // 消費者タスクの開始
    let mut consumer_handles = Vec::with_capacity(consumer_count);
    let rx = Arc::new(tokio::sync::Mutex::new(rx));
    
    for _ in 0..consumer_count {
        let rx = rx.clone();
        let messages_received = messages_received.clone();
        let lag_events = lag_events.clone();
        let buffer_threshold = buffer_size / 5; // 80%埋まったらラグとみなす
        
        let handle = task::spawn(async move {
            while Instant::now() < end_time {
                let mut channel = rx.lock().await;
                
                if let Some(_message) = channel.recv().await {
                    messages_received.fetch_add(1, Ordering::SeqCst);
                    
                    // 消費遅延のシミュレーション
                    drop(channel); // ロックを解放してから遅延
                    time::sleep(Duration::from_millis(consumer_delay)).await;
                    
                    // バッファ使用率チェック
                    let channel = rx.lock().await;
                    if channel.capacity() < buffer_threshold {
                        lag_events.fetch_add(1, Ordering::SeqCst);
                    }
                } else {
                    break; // チャネルが閉じられた
                }
            }
        });
        
        consumer_handles.push(handle);
    }
    
    // 全タスクが終了するのを待つ
    for handle in producer_handles {
        let _ = handle.await;
    }
    
    // 生成者がすべて終了したらチャネルを閉じる
    drop(tx);
    
    // 残りの消費者が終了するのを待つ
    for handle in consumer_handles {
        let _ = handle.await;
    }
    
    // 結果集計
    let duration_ms = start.elapsed().as_millis();
    let messages_sent = messages_sent.load(Ordering::SeqCst);
    let messages_received = messages_received.load(Ordering::SeqCst);
    let messages_dropped = messages_dropped.load(Ordering::SeqCst);
    let lag_events = lag_events.load(Ordering::SeqCst);
    
    // 有効スループットと損失率の計算
    let effective_throughput = if duration_ms > 0 {
        (messages_received as f64) * 1000.0 / (duration_ms as f64)
    } else {
        0.0
    };
    
    let drop_rate = if messages_sent + messages_dropped > 0 {
        (messages_dropped as f64) / ((messages_sent + messages_dropped) as f64)
    } else {
        0.0
    };
    
    info!("テスト結果: バッファ={}, 生成者={}x{}メッセージ/秒, 消費者={}x{}ms", 
         buffer_size, producer_count, producer_rate / producer_count as u64, 
         consumer_count, consumer_delay);
    info!("  送信: {}, 受信: {}, 破棄: {}, ラグイベント: {}", 
         messages_sent, messages_received, messages_dropped, lag_events);
    info!("  有効スループット: {:.2}メッセージ/秒, 損失率: {:.1}%", 
         effective_throughput, drop_rate * 100.0);
    
    Ok(TestResult {
        buffer_size,
        producers: producer_count,
        consumers: consumer_count,
        producer_rate,
        consumer_delay,
        messages_sent,
        messages_received,
        messages_dropped,
        lag_events,
        duration_ms,
        effective_throughput,
        drop_rate: drop_rate * 100.0,
    })
} 