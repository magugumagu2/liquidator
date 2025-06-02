use aave_v3_liquidator::diagnostic_tool::{DiagnosticOpts, run_diagnostics, inspect_channel_buffer, inspect_task_count, inspect_rpc_performance, inspect_json_response_size};
use structopt::StructOpt;
use tracing::{info, warn, error, Level};
use tracing_subscriber::FmtSubscriber;
use anyhow::Result;

#[derive(Debug, StructOpt)]
#[structopt(name = "liquidator-diagnostics", about = "清算ボットの診断ツール")]
enum Cli {
    /// ログファイルからチャネルラグを分析
    #[structopt(name = "analyze")]
    Analyze(DiagnosticOpts),
    
    /// チャネルバッファサイズの検査
    #[structopt(name = "buffer")]
    Buffer {
        /// 検査するバッファサイズ
        #[structopt(short, long, default_value = "5000000")]
        buffer_size: usize,
        
        /// 測定時間（秒）
        #[structopt(short, long, default_value = "600")]
        duration: u64,
    },
    
    /// 並列タスク処理の性能テスト
    #[structopt(name = "tasks")]
    Tasks {
        /// 並列度の最大数
        #[structopt(short, long, default_value = "10")]
        parallelism: usize,
        
        /// テスト時間（秒）
        #[structopt(short, long, default_value = "300")]
        duration: u64,
    },
    
    /// RPC応答時間のテスト
    #[structopt(name = "rpc")]
    Rpc {
        /// RPC呼び出しの間隔（ミリ秒）
        #[structopt(short, long, default_value = "100")]
        interval: u64,
        
        /// テスト実行時間（秒）
        #[structopt(short, long, default_value = "60")]
        duration: u64,
    },

    /// JSONレスポンスサイズの検査
    #[structopt(name = "json-size")]
    JsonSize {
        /// RPCエンドポイントのURL
        #[structopt(short, long)]
        rpc_url: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // トレーシングの初期化
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("トレーシングの初期化に失敗しました");
    
    info!("清算ボット診断ツールを開始します");
    
    match Cli::from_args() {
        Cli::Analyze(opts) => {
            info!("チャネルラグ分析モードで実行します");
            run_diagnostics(opts).await?;
        },
        Cli::Buffer { buffer_size, duration } => {
            info!("チャネルバッファ検査モードで実行します");
            info!("バッファサイズ: {}, 測定時間: {}秒", buffer_size, duration);
            inspect_channel_buffer().await?;
        },
        Cli::Tasks { parallelism, duration } => {
            info!("タスク処理性能テストモードで実行します");
            info!("並列度: {}, テスト時間: {}秒", parallelism, duration);
            inspect_task_count().await?;
        },
        Cli::Rpc { interval, duration } => {
            info!("RPC応答時間テストモードで実行します");
            info!("呼び出し間隔: {}ms, テスト時間: {}秒", interval, duration);
            inspect_rpc_performance().await?;
        },
        Cli::JsonSize { rpc_url } => {
            info!("JSONレスポンスサイズ検査モードで実行します");
            info!("RPC URL: {}", rpc_url);
            inspect_json_response_size(&rpc_url).await?;
        },
    }
    
    info!("診断ツールが正常に終了しました");
    
    Ok(())
} 