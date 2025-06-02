use artemis_core::engine::Engine;
use artemis_core::types::{ExecutorMap, CollectorStream, Collector};
use ethers::{
    providers::{Http, Middleware, Provider, JsonRpcClient, ProviderError},
    middleware::MiddlewareBuilder,
    signers::{LocalWallet, Signer},
    types::U256,
};
use futures::StreamExt;
use clap::Parser;
use std::sync::Arc;
use tokio::time::{self, Duration};
use tracing::{error, info, warn, Level};
use tracing_subscriber::{filter, prelude::*};
use async_trait::async_trait;
use anyhow::Result;

use aave_v3_liquidator::collectors::time_collector::{NewTick, TimeCollector};
use aave_v3_liquidator::strategies::{AaveStrategy, OptimizedSettings, EnvironmentMode};
use aave_v3_liquidator::strategies::types::{Config, Event, Action};
use aave_v3_liquidator::strategies::aave_strategy::Deployment;
use aave_v3_liquidator::executors::protect_executor::ProtectExecutor;
use aave_v3_liquidator::diagnostics::ArtemisChannelMonitor;
use aave_v3_liquidator::metrics::DiagnosticMetrics;

/// CLI Options.
#[derive(Parser, Debug)]
pub struct Args {
    /// Ethereum node WS endpoint.
    #[arg(long)]
    pub archive_rpc: String,

    #[arg(long)]
    pub write_rpc: String,

    /// Private key for sending txs.
    #[arg(long)]
    pub private_key: String,

    /// Percentage of profit to pay in gas.
    #[arg(long)]
    pub bid_percentage: u64,

    #[arg(long)]
    pub deployment: Deployment,

    #[arg(long)]
    pub liquidator_address: String,

    #[arg(long)]
    pub chain_id: u64,

    /// Interval in seconds between polling for new events
    #[arg(long, default_value = "1")]  // 15秒から1秒に短縮（リアルタイムスキャン）
    pub poll_interval_secs: u64,
    
    /// Event channel buffer size (default: 5000000)
    #[arg(long, default_value = "5000000")]
    pub event_buffer_size: usize,
    
    /// Action channel buffer size (default: 5000000)
    #[arg(long, default_value = "5000000")]
    pub action_buffer_size: usize,
    
    /// バックプレッシャーしきい値（チャネルキャパシティの割合、デフォルト: 80）
    #[arg(long, default_value = "80")]
    pub backpressure_threshold: usize,
    
    /// ラグ警告しきい値（このレベルを超えるとログに警告が出ます、デフォルト: 1000）
    #[arg(long, default_value = "1000")]
    pub lag_warning_threshold: usize,
    
    /// モニタリング間隔（秒、デフォルト: 60）
    #[arg(long, default_value = "60")]
    pub monitoring_interval_secs: u64,
    
    /// 高速化モードを有効にする（より積極的なスキャン設定）
    #[arg(long, default_value = "false")]
    pub turbo_mode: bool,
    
    /// デフォルトのポーリング間隔を短縮する（高速化用、デフォルト: false）
    #[arg(long, default_value = "false")]
    pub fast_polling: bool,
    
    /// ログ取得を1ブロックずつ処理する（デフォルト: true）
    #[arg(long, default_value = "true")]
    pub single_block_processing: bool,
}

// カスタムHTTPトランスポートラッパー
#[derive(Debug)]
struct CustomHttpTransport {
    inner: Http,
    max_attempts: usize,
}

impl CustomHttpTransport {
    async fn new(url: &str, max_attempts: usize) -> Result<Self> {
        // カスタム設定でHTTPクライアントを作成
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))  // タイムアウトを30秒に調整
            .tcp_keepalive(Some(Duration::from_secs(30)))
            .pool_max_idle_per_host(10)  // コネクションプールを調整
            .connect_timeout(Duration::from_secs(5))  // 接続タイムアウトを5秒に調整
            .pool_idle_timeout(Some(Duration::from_secs(30)))  // アイドル接続のタイムアウトを30秒に設定
            .tcp_nodelay(true)
            .build()?;
        
        // URLの検証
        let url = reqwest::Url::parse(url).map_err(|e| {
            error!("無効なRPC URL: {}", e);
            anyhow::anyhow!("無効なRPC URL: {}", e)
        })?;

        // 接続テスト
        let http = Http::new_with_client(url.clone(), client.clone());
        let test_provider = Provider::new(http);
        
        // 接続テストを実行（最大3回試行）
        let mut _last_error = None;
        for attempt in 1..=3 {
            match test_provider.get_block_number().await {
                Ok(_) => {
                    info!("RPCエンドポイントへの接続テスト成功: {}", url);
                    break;
                }
                Err(e) => {
                    warn!("RPCエンドポイントへの接続テスト失敗（試行 {}/3）: {}", attempt, e);
                    if attempt == 3 {
                        error!("RPCエンドポイントへの接続テストが3回失敗しました: {}", e);
                        return Err(anyhow::anyhow!("RPCエンドポイントへの接続テスト失敗: {}", e));
                    }
                    _last_error = Some(e);
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
            }
        }

        let http = Http::new_with_client(url, client);
        Ok(Self {
            inner: http,
            max_attempts,
        })
    }
}

#[async_trait]
impl JsonRpcClient for CustomHttpTransport {
    type Error = <Http as JsonRpcClient>::Error;

    async fn request<T, R>(&self, method: &str, params: T) -> std::result::Result<R, Self::Error>
    where
        T: serde::Serialize + Send + Sync + std::fmt::Debug,
        R: serde::de::DeserializeOwned + Send,
    {
        let mut last_error = None;
        for attempt in 1..=self.max_attempts {
            match self.inner.request(method, &params).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    let error_str = e.to_string();
                    // エラーの種類に応じた処理
                    if error_str.contains("EOF while parsing") {
                        warn!("JSON解析エラー（試行 {}/{}）: {}。再試行します...", 
                            attempt, self.max_attempts, e);
                    } else if error_str.contains("Connection refused") {
                        error!("接続拒否エラー（試行 {}/{}）: {}。再試行します...", 
                            attempt, self.max_attempts, e);
                    } else if error_str.contains("timeout") {
                        warn!("タイムアウトエラー（試行 {}/{}）: {}。再試行します...", 
                            attempt, self.max_attempts, e);
                    } else {
                        error!("RPCエラー（試行 {}/{}）: {}。再試行します...", 
                            attempt, self.max_attempts, e);
                    }

                    // 指数バックオフで待機時間を増加
                    let backoff = Duration::from_millis(500 * 2u64.pow(attempt as u32 - 1));
                    tokio::time::sleep(backoff).await;
                    last_error = Some(e);
                    continue;
                }
            }
        }
        Err(last_error.unwrap())
    }
}

// トレイトの定義を直接確認するためのコメント
// ethers::providers::JsonRpcClient トレイトの定義:
// pub trait JsonRpcClient: Send + Sync + std::fmt::Debug {
//     type Error: std::error::Error + Send + Sync;
//     async fn request<T, R>(&self, method: &str, params: T) -> std::result::Result<R, Self::Error>
//     where
//         T: serde::Serialize + Send + Sync + std::fmt::Debug + 'static,
//         R: serde::de::DeserializeOwned + Send + 'static;
// }

// TimeCollectorをEventにマッピングするためのコレクタラッパーを作成
struct EventCollectorWrapper {
    time_collector: TimeCollector,
}

impl EventCollectorWrapper {
    fn new(time_collector: TimeCollector) -> Self {
        Self { time_collector }
    }
}

#[async_trait]
impl Collector<Event> for EventCollectorWrapper {
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, Event>, anyhow::Error> {
        let stream = self.time_collector.get_event_stream().await?;
        let event_stream = stream.map(|new_tick| Event::NewTick(new_tick));
        Ok(Box::pin(event_stream))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Set up tracing and parse args.
    let filter = filter::Targets::new()
        .with_target("artemis_core", Level::INFO)
        .with_target("aave_v3_liquidator", Level::INFO);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    let args = Args::parse();

    // 🚀 環境自動判定と最適化設定
    let env_mode = EnvironmentMode::from_rpc_url(&args.write_rpc);
    let optimized_settings = env_mode.get_optimized_settings();
    
    match env_mode {
        EnvironmentMode::Production => {
            info!("🔥 本番モード検出: サーバー内ローカルRPC使用");
            info!("   📊 最適化設定:");
            info!("      ⚡ タイムアウト: {}秒", optimized_settings.timeout);
            info!("      🔄 リトライ間隔: {}ms", optimized_settings.retry_delay);
            info!("      📦 Multicallチャンクサイズ: {}", optimized_settings.multicall_chunk_size);
            info!("      🧵 最大並列タスク: {}", optimized_settings.max_parallel_tasks);
            info!("      📝 ログレベル: {}", optimized_settings.log_level);
        },
        EnvironmentMode::Development => {
            info!("🔧 開発モード: リモートRPC使用");
            info!("   📊 標準設定でロード");
        }
    }

    // カスタムHTTPトランスポートを使用して接続の信頼性を向上
    let archive_transport = CustomHttpTransport::new(&args.archive_rpc, 3).await?;
    let write_transport = CustomHttpTransport::new(&args.write_rpc, 3).await?;
    
    let mut archive_provider = Provider::new(archive_transport);
    archive_provider.set_interval(Duration::from_millis(500));
    
    let mut write_provider = Provider::new(write_transport);
    write_provider.set_interval(Duration::from_millis(500));

    let wallet: LocalWallet = args
        .private_key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(args.chain_id);
    let address = wallet.address();

    let archive_provider = Arc::new(archive_provider.nonce_manager(address).with_signer(wallet.clone()));
    let write_provider = Arc::new(write_provider.nonce_manager(address).with_signer(wallet.clone()));

    // チャネルモニター用のメトリクスを作成
    let metrics = Arc::new(DiagnosticMetrics::new());
    let channel_monitor = Arc::new(ArtemisChannelMonitor::new(metrics.clone()));
    let monitor_clone = channel_monitor.clone();
    
    // モニタリングタスクを起動
    tokio::spawn(async move {
        let interval = Duration::from_secs(args.monitoring_interval_secs);
        let mut interval_timer = time::interval(interval);
        
        loop {
            interval_timer.tick().await;
            let analysis = monitor_clone.analyze_lag_pattern().await;
            info!("チャネルラグ分析結果:\n{}", analysis);
        }
    });

    // Set up engine.
    let event_buffer_size = args.event_buffer_size * optimized_settings.buffer_multiplier;
    let action_buffer_size = args.action_buffer_size * optimized_settings.buffer_multiplier;
    
    info!("📊 チャネル設定:");
    info!("   イベントバッファ: {} (乗数: {}x)", event_buffer_size, optimized_settings.buffer_multiplier);
    info!("   アクションバッファ: {} (乗数: {}x)", action_buffer_size, optimized_settings.buffer_multiplier);
    
    let mut engine: Engine<Event, Action> = Engine::new()
        .with_event_channel_capacity(event_buffer_size)
        .with_action_channel_capacity(action_buffer_size);
    
    // Set up time collector.
    let poll_interval = if args.fast_polling { 
        1  // fast_pollingモードでは1秒間隔（さらに高速化）
    } else { 
        args.poll_interval_secs 
    };
    
    info!("🚀 ポーリング間隔: {}秒 (fast_polling: {})", poll_interval, args.fast_polling);
    
    let time_collector = TimeCollector::new(poll_interval);
    let time_collector_for_strategy = Arc::new(TimeCollector::new(poll_interval));
    let event_collector = Box::new(EventCollectorWrapper::new(time_collector));
    engine.add_collector(event_collector);

    let config = Config {
        bid_percentage: args.bid_percentage,
        chain_id: args.chain_id,
    };

    let mut strategy = AaveStrategy::new(
        Arc::clone(&archive_provider),
        Arc::clone(&write_provider),
        config,
        args.deployment,
        args.liquidator_address,
        Some(optimized_settings.clone()),
    );
    
    // 高速化モードの適用
    if args.turbo_mode {
        info!("🚀 高速化モードが有効化されました");
        strategy.enable_turbo_mode();
    }
    
    // バックプレッシャー設定
    let backpressure_threshold = args.event_buffer_size * args.backpressure_threshold / 100;
    strategy.set_backpressure_threshold(backpressure_threshold);
    
    // 戦略に作成したTimeCollectorを渡す
    strategy.set_time_collector(time_collector_for_strategy);
    
    // 実験的リアルタイムRPCクライアントの初期化
    info!("🔧 実験的リアルタイムRPC設定の初期化中...");
    if let Err(e) = strategy.init_realtime_client().await {
        warn!("実験的リアルタイムRPCの初期化に失敗: {}。アーカイブRPCのみ使用", e);
    }
    
    // 初回スキャン専用アーカイブRPCクライアントの初期化
    info!("📚 初回スキャン専用アーカイブRPC設定の初期化中...");
    if let Err(e) = strategy.init_initial_scan_client().await {
        warn!("初回スキャン専用アーカイブRPCの初期化に失敗: {}。通常のarchive_clientを使用", e);
    }
    
    // 統合戦略の初期化（エラーで停止しない）
    info!("🔧 統合戦略の初期化を試行中...");
    match strategy.init_integrated_strategy().await {
        Ok(_) => {
            info!("✅ 統合戦略の初期化完了");
        }
        Err(e) => {
            warn!("⚠️ 統合戦略の初期化をスキップ: {}", e);
            warn!("   Redis接続ができないため、スタンドアロンモードで動作します");
            warn!("   💡 Redis使用時の利点:");
            warn!("      - より効率的な借り手キューイング");
            warn!("      - キャッシュによる高速化");
            warn!("      - 分散処理対応");
            warn!("   💡 Redis無しでも基本機能は正常に動作します");
        }
    }
    
    engine.add_strategy(Box::new(strategy));

    let executor = Box::new(ProtectExecutor::new(write_provider.clone(), write_provider.clone()));
    let executor = ExecutorMap::new(executor, |action| match action {
        Action::SubmitTx(tx) => Some(tx),
    });

    engine.add_executor(Box::new(executor));
    info!("Starting engine");

    // エンジン実行時のエラーハンドリング強化
    match engine.run().await {
        Ok(mut set) => {
            while let Some(res) = set.join_next().await {
                info!("res: {:?}", res);
            }
        }
        Err(e) => {
            error!("エンジン起動エラー: {}", e);
            
            // エラーの詳細情報を取得
            if let Some(provider_err) = e.downcast_ref::<ProviderError>() {
                error!("プロバイダエラー詳細: {:?}", provider_err);
                
                // JSON-RPCエラーの一般的な処理
                if provider_err.to_string().contains("JSON-RPC") {
                    error!("JSON-RPCエラー発生 - RPCエンドポイントのレスポンスに問題があります");
                }
            }
            
            // 接続エラーの詳細な処理
            let error_str = e.to_string();
            if error_str.contains("EOF while parsing") || 
               error_str.contains("connection") || 
               error_str.contains("timeout") {
                error!("ネットワーク接続エラーの可能性があります。RPCエンドポイントを確認してください。");
                error!("エラー詳細: {}", error_str);
                error!("提案: RPCエンドポイントのレート制限、ネットワーク安定性、またはJSON応答サイズを確認してください。");
                
                // 特に大きなJSONレスポンスの解析エラーの場合
                if error_str.contains("EOF while parsing") {
                    error!("大きなJSONレスポンスの解析中にエラーが発生しました。RPCエンドポイントからのレスポンスが不完全な可能性があります。");
                    error!("提案: 別のRPCエンドポイントを使用するか、RPCプロバイダに問い合わせてください。");
                }
            }
            
            // ノンブロッキングエラーの場合
            if error_str.contains("would block") {
                error!("非同期処理のブロッキングエラーが発生しました。チャネルバッファサイズを確認してください。");
            }
            
            panic!("清算ボット起動エラー: {}", e);
        }
    }
    
    Ok(())
}
