use super::types::Config;
use crate::collectors::time_collector::{NewTick, TimeCollector};
use anyhow::{anyhow, Result};
use artemis_core::executors::mempool_executor::{GasBidInfo, SubmitTxToMempool};
use artemis_core::types::Strategy;
use async_trait::async_trait;
use bindings_aave::{
    i_aave_oracle::IAaveOracle,
    i_pool_data_provider::IPoolDataProvider,
    ierc20::IERC20,
    pool::{BorrowFilter, Pool, SupplyFilter},
};
use bindings_liquidator::liquidator::Liquidator;
use clap::{Parser, ValueEnum};
use ethers::{
    providers::{Middleware, Provider, ProviderExt},
    types::{
        transaction::eip2718::TypedTransaction, Address, Bytes, H256, I256, U256, U64,
        TransactionRequest,
    },
};
use ethers_contract::Multicall;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{debug, error, info, warn};
use futures::future;
use reqwest::Client;
use serde_json::json;
use redis;

use super::types::{Action, Event};
use super::integrated_approach::{IntegratedLiquidationStrategy, IntegratedStrategyConfig};
use super::priority_queue_manager::BorrowerEvent;

// Addressを簡単にバイト配列に変換するためのトレイト拡張
#[allow(dead_code)]
trait AddressExt {
    fn as_bytes(&self) -> &[u8];
}

#[allow(dead_code)]
impl AddressExt for Address {
    fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Debug, Clone)]
struct DeploymentConfig {
    state_cache_file: String,
    pool_address: Address,
    pool_data_provider: Address,
    oracle_address: Address,
    whype_address: Address,
    multicall3_address: Address,
    liq_paths_config_file: String,
    default_liq_path: String,
    creation_block: u64,
    min_health_factor: U256,
}

#[derive(Debug, Clone, Parser, ValueEnum)]
pub enum Deployment {
    MOCKNET,
    HYPERLEND,
}

pub const LIQUIDATION_CLOSE_FACTOR_THRESHOLD: &str = "950000000000000000";
pub const MAX_LIQUIDATION_CLOSE_FACTOR: u64 = 10000;
pub const DEFAULT_LIQUIDATION_CLOSE_FACTOR: u64 = 5000;

// ログ取得関連の定数
pub const INITIAL_CHUNK_SIZE: u64 = 1;  // リアルタイム用（1ブロックずつ処理）
pub const MIN_CHUNK_SIZE: u64 = 1;      // リアルタイム用（最小チャンクサイズ）

// 初回スキャン専用の定数
pub const INITIAL_SCAN_CHUNK_SIZE: u64 = 5000;  // 初回スキャンは5000ブロックずつ
pub const INITIAL_SCAN_MAX_RETRIES: u32 = 5;    // 初回スキャン用リトライ回数
pub const INITIAL_SCAN_TIMEOUT: u64 = 60;       // 初回スキャンタイムアウト（60秒）

pub const MAX_RETRIES: u32 = 3;
pub const RETRY_DELAY: u64 = 500;          // 1秒 → 500ms に短縮（1秒ブロック対応）
pub const MAIN_RPC_TIMEOUT: u64 = 3;       // 10秒 → 3秒に短縮（高速化）
pub const BACKUP_RPC_TIMEOUT: u64 = 5;     // 15秒 → 5秒に短縮（高速化）

// admin stuff
pub const LOG_BLOCK_RANGE: u64 = 1000;
pub const MULTICALL_CHUNK_SIZE: usize = 30;  // 20から30に増加（1秒ブロック対応）
pub const PRICE_ONE: u64 = 100_000_000;
pub const SCAN_BATCH_SIZE: usize = 150;  // 100 → 150に増加（高速処理）
pub const MAX_PARALLEL_TASKS: usize = 75;  // 50 → 75に増加（1秒ブロック対応）

// サーバー本番運用用の定数（ローカルRPC最適化）
pub const SERVER_MODE_TIMEOUT: u64 = 1;        // サーバー内通信: 1秒タイムアウト
pub const SERVER_MODE_RETRY_DELAY: u64 = 100;  // サーバー内通信: 100msリトライ間隔
pub const SERVER_MULTICALL_CHUNK_SIZE: usize = 50;  // サーバー用: 大きなチャンクサイズ
pub const SERVER_MAX_PARALLEL_TASKS: usize = 100;   // サーバー用: 高並列処理

// Discord通知用WebhookURL
const DISCORD_WEBHOOK_URL: &str = "https://canary.discord.com/api/webhooks/1378380473281151007/OkAPTUr0L8kNys97-WEDlIpsfgiCVuPRbFiGFrFsQgtIkYAx5c0ybYdgmpfBrAW-b1v5";

fn get_deployment_config(deployment: Deployment) -> DeploymentConfig {
    match deployment {
        Deployment::MOCKNET => DeploymentConfig {
            state_cache_file: "borrowers-mocknet.json".to_string(),
            pool_address: Address::from_str("0x32467b43BFa67273FC7dDda0999Ee9A12F2AaA08").unwrap(),
            pool_data_provider: Address::from_str("0x0B306BF915C4d645ff596e518fAf3F9669b97016").unwrap(),
            oracle_address: Address::from_str("0x0E801D84Fa97b50751Dbf25036d067dCf18858bF").unwrap(),
            whype_address: Address::from_str("0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0").unwrap(),
            multicall3_address: Address::from_str("0x720472c8ce72c2A2D711333e064ABD3E6BbEAdd3").unwrap(),
            liq_paths_config_file: "liq_paths.json".to_string(),
            default_liq_path: "hyperswap".to_string(),
            creation_block: 0,
            min_health_factor: U256::from(950000000000000000u64),
        },
        Deployment::HYPERLEND => DeploymentConfig {
            state_cache_file: "borrowers-hyperevm-mainnet.json".to_string(),
            pool_address: Address::from_str("0x00A89d7a5A02160f20150EbEA7a2b5E4879A1A8b").unwrap(),
            pool_data_provider: Address::from_str("0x5481bf8d3946E6A3168640c1D7523eB59F055a29").unwrap(),
            oracle_address: Address::from_str("0xC9Fb4fbE842d57EAc1dF3e641a281827493A630e").unwrap(),
            whype_address: Address::from_str("0x5555555555555555555555555555555555555555").unwrap(),
            multicall3_address: Address::from_str("0xA66AEb1c0A579Ad95bA3940d18FAad02C368A383").unwrap(),
            liq_paths_config_file: "liq_paths.json".to_string(),
            default_liq_path: "kittenswap".to_string(),
            creation_block: 787547,
            min_health_factor: U256::from(950000000000000000u64),
        },
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCache {
    pub last_block_number: u64,
    pub borrowers: HashMap<Address, Borrower>,
}

impl StateCache {
    pub fn new(last_block_number: u64, borrowers: HashMap<Address, Borrower>) -> Self {
        Self {
            last_block_number,
            borrowers,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PoolState {
    pub prices: HashMap<Address, U256>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Borrower {
    address: Address,
    collateral: HashSet<Address>,
    debt: HashSet<Address>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenConfig {
    address: Address,
    a_address: Address,
    decimals: u64,
    ltv: u64,
    liquidation_threshold: u64,
    liquidation_bonus: u64,
    reserve_factor: u64,
    protocol_fee: u64,
}

// トランザクション管理のための構造体
#[derive(Debug, Clone)]
struct PendingLiquidation {
    tx_hash: H256,
    borrower: Address,
    submission_time: SystemTime,
    profit_estimate: I256,
    estimated_gas: U256,
}

// 階層定義のための構造体
#[derive(Debug, Clone)]
struct BorrowerTier {
    name: String,                      // 階層名
    health_factor_range: (U256, U256), // ヘルスファクターの範囲
    scan_interval: Duration,           // スキャン間隔
    parallel_factor: usize,            // 並列度
    last_scan: Option<SystemTime>,     // 前回のスキャン時刻
    borrowers: Vec<Address>,           // この階層の借り手リスト
}

// ノンス管理のための構造体
#[derive(Debug)]
struct NonceManager {
    current_nonce: U256,
    last_update: SystemTime,
}

impl NonceManager {
    async fn new<M: Middleware + 'static>(provider: Arc<M>) -> Result<Self> {
        let sender = provider.default_sender().ok_or(anyhow!("送信者が設定されていません"))?;
        let nonce = provider.get_transaction_count(sender, None).await?;
        
        Ok(Self {
            current_nonce: nonce,
            last_update: SystemTime::now(),
        })
    }
    
    // ノンスを取得して次に使うノンスを返すとともに内部カウンターを更新する
    async fn get_next_nonce<M: Middleware + 'static>(&mut self, provider: Arc<M>) -> Result<U256> {
        let now = SystemTime::now();
        // 5分以上経過していたら再度ノンスを取得する
        if now.duration_since(self.last_update).unwrap().as_secs() > 300 {
            let sender = provider.default_sender().ok_or(anyhow!("送信者が設定されていません"))?;
            self.current_nonce = provider.get_transaction_count(sender, None).await?;
            self.last_update = now;
            
            // 現在のノンスを返して、内部カウンターを増やす
            let result = self.current_nonce;
            self.current_nonce += U256::one();
            
            return Ok(result);
        } 
        
        // 古いノンスの場合は内部カウンターをインクリメントして返す
        let result = self.current_nonce;
        self.current_nonce += U256::one();
        
        Ok(result)
    }
    
    // ノンスを強制的に再同期する（失敗したトランザクションがある場合など）
    async fn resync_nonce<M: Middleware + 'static>(&mut self, provider: Arc<M>) -> Result<()> {
        info!("ノンス強制再同期の実行");
        let sender = provider.default_sender().ok_or(anyhow!("送信者が設定されていません"))?;
        self.current_nonce = provider.get_transaction_count(sender, None).await?;
        self.last_update = SystemTime::now();
        
        info!("ノンスを {}に再同期しました", self.current_nonce);
        Ok(())
    }
}

// 実行設定の構造体
#[derive(Debug)]
struct ExecutionConfig {
    max_concurrent_txs: usize,       // 同時に実行するトランザクションの最大数
    min_profit_threshold: I256,      // 最小利益閾値
    max_gas_price: U256,             // 最大ガス価格
    base_gas_price: U256,            // 基本ガス価格
    gas_price_multiplier: u64,       // ガス価格乗数（パーセント）
    min_profit_multiplier: u64,      // 最小利益乗数（パーセント）
    last_adjustment_time: SystemTime, // 最後に調整した時刻
    adjustment_interval: Duration,    // 調整間隔
    // 🆕 追加: スリッページ保護設定
    slippage_tolerance_bps: u64,     // スリッページ許容度（ベーシスポイント）
    min_profit_strategy: ProfitStrategy, // 最小利益計算戦略
    enable_slippage_protection: bool, // スリッページ保護の有効/無効
    detailed_logging: bool,          // 詳細ログの有効/無効
}

// 🆕 最小利益計算戦略の定義
#[derive(Debug, Clone)]
enum ProfitStrategy {
    FixedAmount(U256),              // 固定額（例：10 USDT0）
    GasMultiplier(f64),             // ガス代の倍数（例：1.5倍）
    LiquidationPercentage(f64),     // 清算額の比率（例：0.5%）
}

impl ExecutionConfig {
    fn new() -> Self {
        Self {
            max_concurrent_txs: 10, // 5 → 10に増加
            min_profit_threshold: I256::from(0),
            max_gas_price: U256::from(10000000000u64), // 5 Gwei → 10 Gwei に増加
            base_gas_price: U256::from(5000000000u64), // 5 Gwei
            gas_price_multiplier: 150, // 200% → 150%に削減（より積極的）
            min_profit_multiplier: 120, // 150% → 120%に削減（より積極的）
            last_adjustment_time: SystemTime::now(),
            adjustment_interval: Duration::from_secs(180), // 5分 → 3分に短縮
            // 🆕 デフォルト設定（後でユーザー入力に基づいて調整）
            slippage_tolerance_bps: 300,  // 3% デフォルト
            min_profit_strategy: ProfitStrategy::GasMultiplier(1.5), // ガス代の1.5倍
            enable_slippage_protection: true,
            detailed_logging: false,     // デフォルトは標準ログ
        }
    }

    // 🆕 ユーザー設定の適用
    fn apply_user_settings(
        &mut self, 
        slippage_bps: Option<u64>,
        profit_strategy: Option<ProfitStrategy>,
        enable_detailed_logs: Option<bool>
    ) {
        if let Some(slippage) = slippage_bps {
            self.slippage_tolerance_bps = slippage;
            if slippage >= 5000 { // 50%以上
                warn!("🚨 危険な高スリッページ設定: {}%", slippage as f64 / 100.0);
                warn!("   ⚠️  このレベルのスリッページは利益をほぼ消す可能性があります");
                warn!("   💡 推奨: 段階的に上げることを検討してください (5% → 10% → 20%)");
            } else if slippage >= 1000 { // 10%以上
                warn!("⚠️ 高スリッページ設定: {}%", slippage as f64 / 100.0);
                warn!("   💡 利益への影響を監視してください");
            } else {
                info!("✅ スリッページ許容度を {}% に設定", slippage as f64 / 100.0);
            }
        }
        
        if let Some(strategy) = profit_strategy {
            self.min_profit_strategy = strategy.clone();
            match strategy {
                ProfitStrategy::FixedAmount(amount) => {
                    info!("💰 最小利益戦略: 固定額 {} USDT0", amount);
                },
                ProfitStrategy::GasMultiplier(multiplier) => {
                    info!("⛽ 最小利益戦略: ガス代の {:.1}倍", multiplier);
                },
                ProfitStrategy::LiquidationPercentage(percent) => {
                    info!("📊 最小利益戦略: 清算額の {:.2}%", percent);
                    if percent < 0.3 {
                        warn!("⚠️ 非常に低い利益閾値です。手数料で利益が消える可能性があります");
                    } else if percent > 2.0 {
                        warn!("⚠️ 非常に高い利益閾値です。清算機会が大幅に減る可能性があります");
                    }
                },
            }
        }
        
        if let Some(detailed) = enable_detailed_logs {
            self.detailed_logging = detailed;
            if detailed {
                info!("📝 詳細ログ: 有効 - 全ての計算過程を表示します");
                info!("   📊 価格情報、債務詳細、清算計算、利益計算の詳細を表示");
                info!("   ⚠️  ログ量が大幅に増加します");
            } else {
                info!("📝 詳細ログ: 無効 - 標準ログのみ表示");
            }
        }
    }

    // 🆕 スリッページを段階的に調整する機能
    fn increase_slippage_gradually(&mut self) -> bool {
        let current_slippage = self.slippage_tolerance_bps;
        let new_slippage = match current_slippage {
            0..=300 => 500,      // 3%以下 → 5%
            301..=500 => 1000,   // 5%以下 → 10%
            501..=1000 => 2000,  // 10%以下 → 20%
            1001..=2000 => 3000, // 20%以下 → 30%
            2001..=3000 => 5000, // 30%以下 → 50%
            _ => return false,   // 50%を超える場合は調整しない
        };
        
        self.slippage_tolerance_bps = new_slippage;
        
        warn!("📈 スリッページ許容度を自動調整: {}% → {}%", 
              current_slippage as f64 / 100.0, 
              new_slippage as f64 / 100.0);
        
        if new_slippage >= 5000 {
            warn!("🚨 最大スリッページレベル到達: 50%");
            warn!("   これ以上の調整は推奨されません");
        } else if new_slippage >= 2000 {
            warn!("⚠️ 高いスリッページレベル: {}%", new_slippage as f64 / 100.0);
            warn!("   利益への影響を注意深く監視してください");
        }
        
        true
    }

    // 🆕 IOAエラー回数に基づくスリッページ調整
    fn adjust_slippage_for_ioa_errors(&mut self, consecutive_ioa_errors: u32) {
        if consecutive_ioa_errors >= 5 {
            let before = self.slippage_tolerance_bps;
            if self.increase_slippage_gradually() {
                info!("🔧 IOAエラー対策: {}回連続エラーによりスリッページ調整", consecutive_ioa_errors);
                info!("   調整前: {}% → 調整後: {}%", 
                      before as f64 / 100.0, 
                      self.slippage_tolerance_bps as f64 / 100.0);
            } else {
                error!("🚨 スリッページ調整不可: 既に最大レベル ({}%)", 
                       self.slippage_tolerance_bps as f64 / 100.0);
                error!("   他の対策を検討してください（異なるDEX、異なるペアなど）");
            }
        }
    }

    // 🆕 動的最小利益計算
    async fn calculate_min_profit_threshold<M: Middleware + 'static>(
        &self,
        estimated_gas: U256,
        gas_price: U256,
        liquidation_amount: U256,
        target_token_decimals: u64
    ) -> I256 {
        match &self.min_profit_strategy {
            ProfitStrategy::FixedAmount(amount) => {
                I256::from_dec_str(&amount.to_string()).unwrap_or(I256::from(0))
            },
            ProfitStrategy::GasMultiplier(multiplier) => {
                let gas_cost = estimated_gas * gas_price;
                // U256を安全にf64に変換
                let gas_cost_u128 = gas_cost.as_u128().min(u64::MAX as u128) as u64;
                let multiplied_cost = gas_cost_u128 as f64 * multiplier;
                let min_profit = U256::from(multiplied_cost as u64);
                
                // ターゲットトークンの単位に調整
                let adjusted_profit = min_profit * U256::from(10).pow(target_token_decimals.into());
                I256::from_dec_str(&adjusted_profit.to_string()).unwrap_or(I256::from(0))
            },
            ProfitStrategy::LiquidationPercentage(percentage) => {
                // U256を安全にf64に変換
                let liquidation_u128 = liquidation_amount.as_u128().min(u64::MAX as u128) as u64;
                let profit_amount = liquidation_u128 as f64 * percentage / 100.0;
                let min_profit = U256::from(profit_amount as u64);
                I256::from_dec_str(&min_profit.to_string()).unwrap_or(I256::from(0))
            }
        }
    }

    // ガス価格の上限を動的に調整
    fn adjust_max_gas_price(&mut self, current_gas_price: U256) {
        let now = SystemTime::now();
        if now.duration_since(self.last_adjustment_time).unwrap() < self.adjustment_interval {
            return;
        }

        // ガス価格をGwei単位に変換
        let current_gas_price_gwei = current_gas_price / U256::exp10(9);
        
        // ガス価格の範囲に応じて上限を設定
        self.max_gas_price = if current_gas_price_gwei <= U256::from(50u64) {
            U256::from(100_000_000_000u64) // 100 Gwei
        } else {
            // 51 Gwei以上は現在のガス価格の2倍
            current_gas_price * U256::from(2)
        };

        self.last_adjustment_time = now;

        info!(
            "ガス価格上限を {} Gwei に調整しました (現在のガス価格: {} Gwei)",
            self.max_gas_price / U256::exp10(9),
            current_gas_price_gwei
        );
    }

    // 利益閾値を動的に調整
    fn adjust_profit_threshold(&mut self, current_gas_price: U256) {
        let now = SystemTime::now();
        if now.duration_since(self.last_adjustment_time).unwrap() < self.adjustment_interval {
            return;
        }

        // ガスコストを計算
        let gas_cost = I256::from(current_gas_price.as_u128());
        
        // 基本利益閾値にガスコストを加算
        let base_threshold = gas_cost;
        
        // 乗数を適用
        let new_threshold = base_threshold * I256::from(self.min_profit_multiplier) / I256::from(100);
        
        self.min_profit_threshold = new_threshold;
        self.last_adjustment_time = now;

        info!(
            "最小利益閾値を {} USD に調整しました (ガスコスト: {} USD)",
            new_threshold,
            gas_cost
        );
    }

    // 市場状況に基づいて設定を調整
    fn adjust_for_market_conditions(&mut self, current_gas_price: U256, network_load: f64) {
        let now = SystemTime::now();
        if now.duration_since(self.last_adjustment_time).unwrap() < self.adjustment_interval {
            return;
        }

        // ネットワーク負荷に基づいて乗数を調整
        if network_load > 0.8 {
            // 高負荷時はより保守的な設定
            self.gas_price_multiplier = 200; // 100%上乗せ（2倍）
            self.min_profit_multiplier = 200; // 100%上乗せ
        } else if network_load < 0.3 {
            // 低負荷時はより積極的な設定
            self.gas_price_multiplier = 200; // 100%上乗せ（2倍）
            self.min_profit_multiplier = 120; // 20%上乗せ
        } else {
            // 通常時はデフォルト設定
            self.gas_price_multiplier = 200; // 100%上乗せ（2倍）
            self.min_profit_multiplier = 150; // 50%上乗せ
        }

        // ガス価格と利益閾値を調整
        self.adjust_max_gas_price(current_gas_price);
        self.adjust_profit_threshold(current_gas_price);

        info!(
            "市場状況に基づいて設定を調整しました (ネットワーク負荷: {:.2})",
            network_load
        );
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct AaveStrategy<M> {
    /// Ethers client.
    archive_client: Arc<M>,
    write_client: Arc<M>,
    /// 実験的リアルタイム用クライアント (環境によって自動選択: localhost/外部IP)
    realtime_client: Option<Arc<Provider<ethers::providers::Http>>>,
    /// 初回スキャン専用アーカイブRPCクライアント (https://rpc.hyperlend.finance/archive)
    initial_scan_client: Option<Arc<Provider<ethers::providers::Http>>>,
    /// Amount of profits to bid in gas
    bid_percentage: u64,
    last_block_number: u64,
    borrowers: HashMap<Address, Borrower>,
    tokens: HashMap<Address, TokenConfig>,
    chain_id: u64,
    config: DeploymentConfig,
    liquidator: Address,
    pool_state: PoolState,
    scan_state: ScanState,
    at_risk_borrowers: HashMap<Address, U256>,
    time_collector: Option<Arc<TimeCollector>>,
    pending_liquidations: Vec<PendingLiquidation>,
    nonce_manager: Option<NonceManager>,
    execution_config: ExecutionConfig,
    // 統合戦略
    integrated_strategy: Option<Arc<IntegratedLiquidationStrategy>>,
    // バックプレッシャー関連
    backpressure_threshold: usize,
    is_backpressure_active: bool,
    // 初回スキャン完了フラグ
    initial_scan_completed: bool,
    // 🆕 RPCエラー監視とフォールバック
    consecutive_rpc_errors: u32,           // 連続RPCエラー回数
    rpc_error_threshold: u32,              // エラー閾値（この回数を超えるとフォールバック）
    is_using_archive_fallback: bool,       // アーカイブRPCフォールバックフラグ
    last_successful_rpc_time: Option<SystemTime>, // 最後に成功したRPC呼び出し時刻
}

#[derive(Debug)]
struct ScanState {
    base_interval: u64,
    #[allow(dead_code)]
    extended_interval: u64,
    reduced_interval: u64,
    consecutive_empty_scans: u32,
    #[allow(dead_code)]
    consecutive_empty_threshold: u32,
    #[allow(dead_code)]
    underwater_threshold: u32,
    current_interval: u64,
    last_underwater_count: u32,
    tiers: Vec<BorrowerTier>,           // 追加: 借り手階層情報
    last_tier_update: Option<SystemTime>, // 追加: 最後に階層を更新した時刻
}

impl<M: Middleware + 'static> AaveStrategy<M> {
    pub fn new(
        archive_client: Arc<M>,
        write_client: Arc<M>,
        config: Config,
        deployment: Deployment,
        liquidator_address: String,
        optimized_settings: Option<OptimizedSettings>,
    ) -> Self {
        let deployment_config = get_deployment_config(deployment);
        
        let mut scan_state = ScanState {
            base_interval: 1, // 8秒 → 1秒に短縮（1秒ブロック対応）
            extended_interval: 10, // 60秒 → 10秒に短縮
            reduced_interval: 1, // 3秒 → 1秒に短縮（最速スキャン）
            consecutive_empty_scans: 0,
            consecutive_empty_threshold: 3,
            underwater_threshold: 10,
            current_interval: 1, // 8秒 → 1秒に短縮（1秒ブロック対応）
            last_underwater_count: 0,
            tiers: vec![],
            last_tier_update: None,
        };
        
        // 最適化設定に基づく調整
        if let Some(ref settings) = optimized_settings {
            info!("🚀 最適化設定を適用中:");
            info!("   タイムアウト: {}秒", settings.timeout);
            info!("   並列度: {}", settings.max_parallel_tasks);
            
            // 本番環境の場合はより高速な設定を適用
            if settings.max_parallel_tasks >= SERVER_MAX_PARALLEL_TASKS {
                info!("🔥 本番高性能モード適用");
                scan_state.base_interval = 1;     // 1秒維持
                scan_state.reduced_interval = 1;  // 1秒維持（最速）
            }
        }
        
        // 借り手層の初期設定（1秒ブロック対応版）
        scan_state.tiers = vec![
            BorrowerTier {
                name: "クリティカル".to_string(),
                health_factor_range: (U256::zero(), U256::from(100000000000000000u64)), // < 0.1
                scan_interval: Duration::from_millis(500), // 2秒 → 0.5秒に短縮（超高速）
                parallel_factor: if optimized_settings.as_ref().map_or(false, |s| s.max_parallel_tasks >= 100) { 16 } else { 12 }, // 本番環境で増加
                last_scan: None,
                borrowers: vec![],
            },
            BorrowerTier {
                name: "高リスク".to_string(),
                health_factor_range: (U256::from(100000000000000000u64), U256::from(1000000000000000000u64)), // 0.1-1.0
                scan_interval: Duration::from_secs(1), // 8秒 → 1秒に短縮（ブロック毎）
                parallel_factor: if optimized_settings.as_ref().map_or(false, |s| s.max_parallel_tasks >= 100) { 12 } else { 10 }, // 本番環境で増加
                last_scan: None,
                borrowers: vec![],
            },
            BorrowerTier {
                name: "中リスク".to_string(),
                health_factor_range: (U256::from(1000000000000000000u64), U256::from(1200000000000000000u64)), // 1.0-1.2
                scan_interval: Duration::from_secs(3), // 30秒 → 3秒に短縮
                parallel_factor: if optimized_settings.as_ref().map_or(false, |s| s.max_parallel_tasks >= 100) { 10 } else { 8 }, // 本番環境で増加
                last_scan: None,
                borrowers: vec![],
            },
            BorrowerTier {
                name: "低リスク".to_string(),
                health_factor_range: (U256::from(1200000000000000000u64), U256::from(1500000000000000000u64)), // 1.2-1.5
                scan_interval: Duration::from_secs(10), // 120秒 → 10秒に短縮
                parallel_factor: if optimized_settings.as_ref().map_or(false, |s| s.max_parallel_tasks >= 100) { 8 } else { 6 }, // 本番環境で増加
                last_scan: None,
                borrowers: vec![],
            },
            BorrowerTier {
                name: "安全".to_string(),
                health_factor_range: (U256::from(1500000000000000000u64), U256::max_value()), // > 1.5
                scan_interval: Duration::from_secs(30), // 300秒 → 30秒に短縮
                parallel_factor: if optimized_settings.as_ref().map_or(false, |s| s.max_parallel_tasks >= 100) { 6 } else { 4 }, // 本番環境で増加
                last_scan: None,
                borrowers: vec![],
            },
        ];
        
        let mut execution_config = ExecutionConfig::new();
        
        // 最適化設定に基づく実行設定の調整
        if let Some(ref settings) = optimized_settings {
            if settings.max_parallel_tasks >= SERVER_MAX_PARALLEL_TASKS {
                // 本番環境向け最適化
                execution_config.max_concurrent_txs = 25; // 20 → 25に増加
                execution_config.gas_price_multiplier = 110; // より積極的
                execution_config.min_profit_multiplier = 100; // より積極的
                execution_config.adjustment_interval = Duration::from_secs(30); // 30秒に短縮
                info!("🔥 本番実行設定: 並列TX={}, ガス乗数={}%", 
                      execution_config.max_concurrent_txs, execution_config.gas_price_multiplier);
            }
        }
        
        // 🆕 ユーザー指定設定の適用
        execution_config.apply_user_settings(
            Some(500),  // 5%スリッページ許容度（50%は危険すぎるため5%で開始）
            Some(ProfitStrategy::LiquidationPercentage(0.5)), // 清算額の0.5%
            Some(true)  // 詳細ログ有効
        );
        
        info!("🎯 ユーザー設定が適用されました:");
        info!("   📊 最小利益戦略: 清算額の0.5%");
        info!("   📝 ログレベル: 詳細（DETAILED）");
        info!("   💹 スリッページ許容度: 5% (安全な範囲で開始)");
        info!("   ⚠️  注意: 50%スリッページは利益をほぼ消すため、5%で開始します");
        
        Self {
            archive_client,
            write_client,
            realtime_client: None,
            initial_scan_client: None,
            bid_percentage: config.bid_percentage,
            last_block_number: deployment_config.creation_block,
            borrowers: HashMap::new(),
            tokens: HashMap::new(),
            chain_id: config.chain_id,
            config: deployment_config,
            liquidator: liquidator_address.parse().unwrap_or_default(),
            pool_state: PoolState {
                prices: HashMap::new(),
            },
            scan_state,
            at_risk_borrowers: HashMap::new(),
            time_collector: None,
            pending_liquidations: vec![],
            nonce_manager: None,
            execution_config,
            integrated_strategy: None,
            backpressure_threshold: if optimized_settings.as_ref().map_or(false, |s| s.max_parallel_tasks >= 100) { 20 } else { 10 }, // 本番環境で増加
            is_backpressure_active: false,
            initial_scan_completed: false,
            consecutive_rpc_errors: 0,
            rpc_error_threshold: 5,
            is_using_archive_fallback: false,
            last_successful_rpc_time: None,
        }
    }
    
    // 統合戦略を初期化するメソッド
    pub async fn init_integrated_strategy(&mut self) -> Result<()> {
        info!("統合戦略の初期化を開始します");
        
        // Redis接続をテスト
        let redis_available = self.test_redis_connection().await;
        
        if !redis_available {
            warn!("⚠️ Redis接続不可: 統合戦略を無効化してスタンドアロンモードで続行");
            return Ok(()); // エラーにせず、統合戦略なしで続行
        }
        
        // 統合戦略の設定（高速化版）
        let config = IntegratedStrategyConfig {
            redis_url: std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            cache_prefix: "liquidator".to_string(),
            maintenance_interval: 120, // 5分 → 2分に短縮
            scan_batch_size: 100, // 50 → 100に増加
            parallelism: 8, // 4 → 8に増加
            max_concurrent_batches: 20, // 10 → 20に増加
        };

        // 統合戦略のインスタンスを作成
        match IntegratedLiquidationStrategy::new(config).await {
            Ok(strategy) => {
                self.integrated_strategy = Some(Arc::new(strategy));
                info!("✅ 統合戦略の初期化が完了しました");
            },
            Err(e) => {
                warn!("⚠️ 統合戦略の初期化に失敗: {}。スタンドアロンモードで続行", e);
                // エラーを返さず、統合戦略なしで続行
            }
        }
        
        Ok(())
    }
    
    // Redis接続テスト
    async fn test_redis_connection(&self) -> bool {
        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
        
        match redis::Client::open(redis_url.as_str()) {
            Ok(client) => {
                match client.get_async_connection().await {
                    Ok(mut conn) => {
                        match redis::cmd("PING").query_async::<_, String>(&mut conn).await {
                            Ok(response) if response == "PONG" => {
                                info!("✅ Redis接続確認OK: {}", redis_url);
                                true
                            },
                            Ok(_) => {
                                warn!("❌ Redis PINGレスポンス異常");
                                false
                            },
                            Err(e) => {
                                warn!("❌ Redis PING失敗: {}", e);
                                false
                            }
                        }
                    },
                    Err(e) => {
                        warn!("❌ Redis接続失敗: {}", e);
                        false
                    }
                }
            },
            Err(e) => {
                warn!("❌ Redisクライアント作成失敗: {}", e);
                false
            }
        }
    }
    
    // 既存のset_time_collector実装
    pub fn set_time_collector(&mut self, collector: Arc<TimeCollector>) {
        self.time_collector = Some(collector);
    }

    const USDT0_ADDRESS: &'static str = "0xB8CE59FC3717ada4C02eaDF9682A9e934F625ebb";

    fn adjust_scan_interval(&mut self, critical_borrowers_count: usize) -> Result<()> {
        self.scan_state.last_underwater_count = critical_borrowers_count as u32;
        
        if critical_borrowers_count > 0 {
            self.scan_state.consecutive_empty_scans = 0;
            self.scan_state.current_interval = self.scan_state.reduced_interval;
            info!("危険ゾーンの借り手が検出されました ({}). スキャン間隔を {}秒に短縮します", 
                  critical_borrowers_count, self.scan_state.current_interval);
        } else {
            // 間隔延長はしない設計に変更
            self.scan_state.current_interval = self.scan_state.base_interval;
            info!("危険ゾーンの借り手はいません。基本スキャン間隔 {}秒を維持します", 
                  self.scan_state.current_interval);
        }
        
        if let Some(collector) = &self.time_collector {
            collector.adjust_interval(self.scan_state.current_interval)?;
        }
        
        Ok(())
    }

    fn encode_path_for_liquidation(&self, collateral: &Address, debt: &Address) -> Vec<u8> {
        // IOA対策とペア不存在対策付きパス作成関数を使用
        match create_swap_path_with_ioa_protection(*collateral, *debt, &self.config.default_liq_path) {
            Ok(path) => path,
            Err(e) => {
                error!("全てのパス作成戦略が失敗: {}。緊急フォールバック", e);
                // 緊急フォールバック：最小限の20バイトパス（直接清算のみ）
                let mut emergency_path = Vec::new();
                emergency_path.extend_from_slice(&collateral.0);
                emergency_path
            }
        }
    }

    // バックプレッシャーしきい値を設定するメソッドを追加
    pub fn set_backpressure_threshold(&mut self, threshold: usize) {
        self.backpressure_threshold = threshold;
        info!("バックプレッシャーしきい値を {}に設定しました", threshold);
    }

    // バックプレッシャー状態を確認するメソッド
    fn check_backpressure(&mut self) -> bool {
        let current_pending = self.pending_liquidations.len();
        
        if current_pending >= self.backpressure_threshold {
            if !self.is_backpressure_active {
                warn!("🚨 バックプレッシャー有効化: 保留中の清算 {} 件が閾値 {} を超過", 
                      current_pending, self.backpressure_threshold);
                self.is_backpressure_active = true;
            }
            true
        } else {
            if self.is_backpressure_active {
                info!("✅ バックプレッシャー解除: 保留中の清算 {} 件が正常レベルに回復", current_pending);
                self.is_backpressure_active = false;
            }
            false
        }
    }

    // 🆕 RPCエラー監視とフォールバック管理
    fn record_rpc_success(&mut self) {
        if self.consecutive_rpc_errors > 0 {
            info!("✅ RPC復旧: 連続エラー {} 回から復旧", self.consecutive_rpc_errors);
        }
        self.consecutive_rpc_errors = 0;
        self.last_successful_rpc_time = Some(SystemTime::now());
        
        // フォールバックからの復旧判定
        if self.is_using_archive_fallback {
            info!("🔄 リアルタイムRPCが復旧しました。通常処理に戻します");
            self.is_using_archive_fallback = false;
        }
    }

    fn record_rpc_error(&mut self, error: &str) {
        self.consecutive_rpc_errors += 1;
        
        if self.consecutive_rpc_errors >= self.rpc_error_threshold {
            if !self.is_using_archive_fallback {
                warn!("🚨 リアルタイムRPCエラーが {} 回連続発生", self.consecutive_rpc_errors);
                warn!("   最新エラー: {}", error);
                warn!("   📚 アーカイブRPCフォールバックを有効化します");
                self.is_using_archive_fallback = true;
            }
        } else {
            warn!("⚠️ RPC エラー ({}/{}): {}", 
                  self.consecutive_rpc_errors, self.rpc_error_threshold, error);
        }
    }

    fn should_use_archive_fallback(&self) -> bool {
        self.is_using_archive_fallback && self.initial_scan_client.is_some()
    }

    fn get_fallback_archive_client(&self) -> Option<Arc<Provider<ethers::providers::Http>>> {
        if self.should_use_archive_fallback() {
            self.initial_scan_client.clone()
        } else {
            None
        }
    }
}

// LiquidationOpportunity構造体の前に以下のコードを追加

// 型推論の問題を解決するためのヘルパー関数と型の定義を追加
type TaskResult = tokio::task::JoinHandle<Result<Option<LiquidationOpportunity>>>;

// 明示的な型パラメータを使用したプロセッサー関数
async fn process_tasks(tasks: Vec<TaskResult>) -> Vec<LiquidationOpportunity> {
    let mut results = Vec::new();
    for task_result in futures::future::join_all(tasks).await {
        if let Ok(Ok(Some(op))) = task_result {
            // 明示的な型の比較
            let zero = I256::from(0i64);
            if op.profit_usd.gt(&zero) {
                results.push(op);
            }
        }
    }
    results
}

#[derive(Clone, Debug)]
struct LiquidationOpportunity {
    borrower: Address,
    collateral: Address,
    collateral_to_liquidate: U256,
    debt: Address,
    debt_to_cover: U256,
    profit_usd: I256,
}

impl LiquidationOpportunity {
    // 明示的に型を指定した利益の確認メソッド
    fn has_profit(&self) -> bool {
        let zero = I256::from(0i64);
        self.profit_usd.gt(&zero)
    }
}

// tokio::spawnのジェネリックパラメータを明示する関数として、タスク生成を分離
async fn run_liquidation_task<T: Middleware + 'static>(
    borrower: Address,
    collateral: Address,
    debt: Address,
    health_factor: U256,
    pool_data: IPoolDataProvider<T>,
    pool_state: PoolState,
    write_client: Arc<T>,
    liquidator: Address,
    config: DeploymentConfig,
    tokens: HashMap<Address, TokenConfig>,
    _chain_id: u64,  // 未使用変数の先頭にアンダースコア
) -> Result<Option<LiquidationOpportunity>> {
    info!("🔍 清算評価開始: 借り手 {:?} ({} → {})", borrower, collateral, debt);
    
    // 📊 詳細ログ設定の確認（コンパイル時にデフォルトでtrueにする）
    let detailed_logging = true; // 詳細ログ有効
    
    if detailed_logging {
        info!("📋 詳細計算ログ開始");
        info!("   👤 借り手: {:?}", borrower);
        info!("   💎 担保トークン: {:?}", collateral);
        info!("   💸 債務トークン: {:?}", debt);
        info!("   ❤️  ヘルスファクター: {}", health_factor);
    }
    
    // 🔒 清算実行前の厳格なバリデーション
    match AaveStrategy::<T>::validate_liquidation_before_execution(
        borrower,
        collateral, 
        debt,
        health_factor,
        &pool_data,
        write_client.clone(),
        &config,
    ).await {
        Ok(true) => {
            info!("✅ バリデーション通過: {} → {}", collateral, debt);
        },
        Ok(false) => {
            info!("❌ バリデーション失敗: 清算をスキップ");
            return Ok(None);
        },
        Err(e) => {
            warn!("⚠️ バリデーションエラー: {}。清算をスキップ", e);
            return Ok(None);
        }
    }

    // 必要なデータを取得
    let collateral_asset_price = pool_state
        .prices
        .get(&collateral)
        .ok_or(anyhow!("No collateral price"))?;
    
    let debt_asset_price = pool_state
        .prices
        .get(&debt)
        .ok_or(anyhow!("No debt price"))?;
    
    let collateral_config = tokens
        .get(&collateral)
        .ok_or(anyhow!("Failed to get collateral address"))?;
    
    let debt_config = tokens
        .get(&debt)
        .ok_or(anyhow!("Failed to get debt address"))?;
    
    if detailed_logging {
        info!("💰 価格情報:");
        info!("   担保価格: {} ({})", collateral_asset_price, collateral_config.decimals);
        info!("   債務価格: {} ({})", debt_asset_price, debt_config.decimals);
        info!("   清算ボーナス: {}%", collateral_config.liquidation_bonus as f64 / 100.0);
    }
    
    let collateral_unit = U256::from(10).pow(collateral_config.decimals.into());
    let debt_unit = U256::from(10).pow(debt_config.decimals.into());
    let liquidation_bonus = collateral_config.liquidation_bonus;
    let a_token = IERC20::new(collateral_config.a_address.clone(), write_client.clone());

    let (_, stable_debt, variable_debt, _, _, _, _, _, _) = pool_data
        .get_user_reserve_data(debt, borrower)
        .await?;
    
    if detailed_logging {
        info!("📊 債務詳細:");
        info!("   安定債務: {}", stable_debt);
        info!("   変動債務: {}", variable_debt);
        info!("   合計債務: {}", stable_debt + variable_debt);
    }
    
    let close_factor = if health_factor.gt(&U256::from(LIQUIDATION_CLOSE_FACTOR_THRESHOLD)) {
        U256::from(DEFAULT_LIQUIDATION_CLOSE_FACTOR)
    } else {
        U256::from(MAX_LIQUIDATION_CLOSE_FACTOR)
    };

    if detailed_logging {
        info!("⚖️ 清算係数: {}% ({})", 
              close_factor.as_u64() as f64 / 100.0,
              if close_factor.as_u64() == DEFAULT_LIQUIDATION_CLOSE_FACTOR { "デフォルト" } else { "最大" });
    }

    let mut debt_to_cover =
        (stable_debt + variable_debt) * close_factor / MAX_LIQUIDATION_CLOSE_FACTOR;
    
    let base_collateral = (debt_asset_price * debt_to_cover * debt_unit)
        / (collateral_asset_price * collateral_unit);
    
    let mut collateral_to_liquidate = base_collateral * U256::from(liquidation_bonus) / U256::from(10000);
    let user_collateral_balance = a_token.balance_of(borrower).await?;

    if detailed_logging {
        info!("🧮 清算計算:");
        info!("   カバーする債務: {}", debt_to_cover);
        info!("   基本担保額: {}", base_collateral);
        info!("   ボーナス込み担保: {}", collateral_to_liquidate);
        info!("   借り手担保残高: {}", user_collateral_balance);
    }

    if collateral_to_liquidate > user_collateral_balance {
        collateral_to_liquidate = user_collateral_balance;
        debt_to_cover = (collateral_asset_price * collateral_to_liquidate * debt_unit)
            / (debt_asset_price * collateral_unit * U256::from(liquidation_bonus) / U256::from(10000));
        
        if detailed_logging {
            warn!("⚠️ 担保不足により調整:");
            warn!("   調整後担保: {}", collateral_to_liquidate);
            warn!("   調整後債務: {}", debt_to_cover);
        }
    }

    let mut op = LiquidationOpportunity {
        borrower,
        collateral,
        collateral_to_liquidate,
        debt,
        debt_to_cover,
        profit_usd: I256::from(0i64),
    };

    // 🛡️ 安全なパス作成（IOA対策付き）
    let usdt0_address = Address::from_str(AaveStrategy::<T>::USDT0_ADDRESS)?;
    
    let swap_path = match create_swap_path_with_ioa_protection(collateral, debt, &config.default_liq_path) {
        Ok(path) => {
            if detailed_logging {
                info!("🛤️ スワップパス作成成功:");
                info!("   パス長: {}バイト", path.len());
                info!("   DEX: {}", config.default_liq_path);
                info!("   パスデータ: {:?}", path);
            }
            path
        },
        Err(e) => {
            warn!("🚨 スワップパス作成失敗: {}。清算をスキップ", e);
            return Ok(None);
        }
    };
    
    // 清算シミュレーション（call実行）
    if detailed_logging {
        info!("🎮 清算シミュレーション開始:");
        info!("   清算コントラクト: {:?}", liquidator);
        info!("   最終トークン: {:?}", usdt0_address);
    }
    
    let liquidator_contract = Liquidator::new(liquidator, write_client.clone());
    let contract_call = liquidator_contract.liquidate(
        op.collateral,
        op.debt,
        op.borrower,
        op.debt_to_cover,
        Bytes::from(swap_path),
        config.default_liq_path.clone(),
        usdt0_address,
    );

    let (final_token, gain) = match contract_call.call().await {
        Ok(result) => {
            if detailed_logging {
                info!("✅ シミュレーション成功:");
                info!("   最終トークン: {:?}", result.0);
                info!("   生のゲイン: {}", result.1);
            }
            result
        },
        Err(e) => {
            warn!("🚨 清算シミュレーション失敗: {}。スキップ", e);
            return Ok(None);
        }
    };

    let final_token_price = pool_state
        .prices
        .get(&final_token)
        .ok_or(anyhow!("No price found for final token"))?;

    op.profit_usd = gain * I256::from_dec_str(&final_token_price.to_string())? / I256::from(PRICE_ONE);

    if detailed_logging {
        info!("💵 利益計算:");
        info!("   最終トークン価格: {}", final_token_price);
        info!("   USD換算利益: {}", op.profit_usd);
        info!("   清算額: {}", debt_to_cover);
        let liquidation_amount_usd = debt_to_cover * debt_asset_price / debt_unit;
        let profit_percentage = if liquidation_amount_usd > U256::zero() {
            (op.profit_usd.abs().as_u128() as f64) / (liquidation_amount_usd.as_u128() as f64) * 100.0
        } else {
            0.0
        };
        info!("   利益率: {:.2}%", profit_percentage);
        info!("   最小利益閾値: 0.5% ({})", liquidation_amount_usd / U256::from(200)); // 0.5% = 1/200
    }

    // 明示的な型比較
    let zero = I256::from(0i64);
    if op.profit_usd.gt(&zero) {
        info!("💰 利益ある清算発見: 借り手 {:?}, 利益 {:?} USD", borrower, op.profit_usd);
        
        // 最小利益チェック（清算額の0.5%）
        let liquidation_amount_usd = debt_to_cover * debt_asset_price / debt_unit;
        let min_profit_threshold = liquidation_amount_usd / U256::from(200); // 0.5% = 1/200
        let profit_u256 = U256::from(op.profit_usd.abs().as_u128());
        
        if profit_u256 >= min_profit_threshold {
            if detailed_logging {
                info!("✅ 最小利益要件クリア: {} >= {}", profit_u256, min_profit_threshold);
            }
            return Ok(Some(op));
        } else {
            if detailed_logging {
                warn!("❌ 最小利益要件未達: {} < {} (0.5%)", profit_u256, min_profit_threshold);
            }
            info!("📉 利益不足でスキップ: 借り手 {:?}, 利益 {:?} USD (要件: {} USD)", 
                  borrower, op.profit_usd, min_profit_threshold);
        }
    } else {
        info!("📉 利益なし: 借り手 {:?}, 損失 {:?} USD", borrower, op.profit_usd);
    }
    
    Ok(None)
}

#[async_trait]
impl<M: Middleware + 'static> Strategy<Event, Action> for AaveStrategy<M> {
    async fn sync_state(&mut self) -> Result<()> {
        info!("syncing state");

        self.update_token_configs().await?;
        self.approve_tokens().await?;
        self.load_cache().await?;
        self.update_state().await?;

        // ノンスマネージャーの初期化
        if self.nonce_manager.is_none() {
            self.nonce_manager = Some(NonceManager::new(self.write_client.clone()).await?);
        }

        info!("done syncing state");
        Ok(())
    }

    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        match event {
            Event::NewTick(block) => self.process_new_tick_event(block).await,
        }
    }
}

impl<M: Middleware + 'static> AaveStrategy<M> {
    async fn process_new_tick_event(&mut self, _event: NewTick) -> Vec<Action> {
        info!("新しいティックイベントを受信しました");
        
        // バックプレッシャーの確認
        if self.check_backpressure() {
            warn!("バックプレッシャーが有効です。処理を制限します");
            if let Err(e) = self.check_pending_liquidations().await {
                error!("ペンディングトランザクション確認エラー: {}", e);
            }
            return vec![];
        }

        // 現在のガス価格を取得
        let current_gas_price = match self.write_client.get_gas_price().await {
            Ok(price) => price,
            Err(e) => {
                error!("ガス価格取得エラー: {}", e);
                return vec![];
            }
        };

        // ネットワーク負荷を取得
        let network_load = match self.get_network_load().await {
            Ok(load) => load,
            Err(e) => {
                error!("ネットワーク負荷取得エラー: {}", e);
                0.5 // デフォルト値
            }
        };

        // 市場状況に基づいて設定を調整
        self.execution_config.adjust_for_market_conditions(current_gas_price, network_load);

        // ガス価格チェック
        if current_gas_price > self.execution_config.max_gas_price {
            warn!(
                "現在のガス価格 ({} Gwei) が上限 ({} Gwei) を超えています。スキャンをスキップします",
                current_gas_price / U256::exp10(9),
                self.execution_config.max_gas_price / U256::exp10(9)
            );
            return vec![];
        }
        
        // 統合戦略の初期化チェック
        if self.integrated_strategy.is_none() {
            info!("統合戦略を初期化しています");
            match self.init_integrated_strategy().await {
                Ok(_) => info!("統合戦略の初期化に成功しました"),
                Err(e) => {
                    error!("統合戦略の初期化に失敗しました: {}", e);
                    // 初期化失敗しても従来の方法で続行
                }
            }
        }
        
        // ノンスマネージャーの初期化確認
        if self.nonce_manager.is_none() {
            info!("ノンスマネージャーを初期化しています");
            match NonceManager::new(self.write_client.clone()).await {
                Ok(nm) => self.nonce_manager = Some(nm),
                Err(e) => {
                    error!("ノンスマネージャーの初期化に失敗しました: {}", e);
                    return vec![];
                }
            }
        }
        
        // 既存のペンディングトランザクションを確認
        if let Err(e) = self.check_pending_liquidations().await {
            error!("ペンディングトランザクション確認エラー: {}", e);
        }
        
        // 最大同時実行数と現在のペンディング数を確認
        let available_slots = self.execution_config.max_concurrent_txs.saturating_sub(self.pending_liquidations.len());
        
        if available_slots == 0 {
            info!("最大トランザクション数 ({}) に達しています。スキャンをスキップします", 
                  self.execution_config.max_concurrent_txs);
            return vec![];
        }
        
        // ステータス更新とリスクのある借り手の検出
        if let Err(e) = self.update_state().await {
            error!("ステータス更新エラー: {}", e);
            return vec![];
        }

        info!("借り手の総数: {}", self.borrowers.len());
        
        // 統合戦略が有効な場合は、そちらを使用
        if let Some(ref integrated_strategy) = self.integrated_strategy.clone() {
            // 統合戦略のメンテナンスを実行
            if let Err(e) = integrated_strategy.run_maintenance().await {
                warn!("統合戦略メンテナンスエラー: {}", e);
            }
            
            // 借り手の状態更新
            for (borrower, details) in &self.borrowers {
                // 同時に債務と担保を持つ借り手のみを対象に
                if !details.debt.is_empty() && !details.collateral.is_empty() {
                    // 借り手をスキャン対象に追加
                    if let Err(e) = integrated_strategy.process_event(BorrowerEvent::NewBorrow(*borrower, Address::zero())).await {
                        warn!("借り手のキュー追加エラー: {}", e);
                    }
                }
            }
            
            // 水没借り手やリスクのある借り手を検出
            let at_risk_borrowers = match self.get_underwater_and_at_risk_borrowers().await {
                Ok(borrowers) => borrowers,
                Err(e) => {
                    error!("リスク借り手検出エラー: {}", e);
                    vec![]
                }
            };
            
            // アットリスクの借り手数をモニタリング
            let critical_count = at_risk_borrowers.iter()
                .filter(|(_, hf)| hf < &U256::from(1000000000000000000u64)) // 1.0未満
                .count();
            
            if let Err(e) = self.adjust_scan_interval(critical_count) {
                warn!("スキャン間隔調整エラー: {}", e);
            }
            
            // バックプレッシャー条件を再確認
            if self.check_backpressure() {
                warn!("処理中にバックプレッシャー条件が満たされました。処理を制限します");
                // 既存のキュー内の重要な借り手のみを処理
                let limited_borrowers: Vec<_> = at_risk_borrowers.iter()
                    .filter(|(_, hf)| hf < &U256::from(950000000000000000u64)) // 0.95未満の重要な借り手のみ
                    .take(10) // 最大10件に制限
                    .collect();
                
                info!("バックプレッシャー制限により、{}件の重要な借り手のみを処理します", limited_borrowers.len());
                
                for (borrower, hf) in limited_borrowers {
                    let event = BorrowerEvent::HealthFactorUpdate(*borrower, *hf);
                    if let Err(e) = integrated_strategy.process_event(event).await {
                        warn!("重要イベント処理エラー (借り手: {:?}): {}", borrower, e);
                    }
                }
                
                return vec![];
            }
            
            // 借り手のイベント処理（通常処理）
            for (borrower, hf) in at_risk_borrowers {
                // ヘルスファクターに基づいてイベント生成
                let event = if hf < U256::from(1000000000000000000u64) { // 1.0
                    BorrowerEvent::HealthFactorUpdate(borrower, hf)
                } else {
                    continue; // 1.0以上はスキップ
                };
                
                if let Err(e) = integrated_strategy.process_event(event).await {
                    warn!("イベント処理エラー (借り手: {:?}): {}", borrower, e);
                }
            }
            
            // 次にスキャンすべきバッチを取得
            match integrated_strategy.get_next_batch_to_scan().await {
                Ok(batch) => {
                    if batch.is_empty() {
                        info!("スキャン対象の借り手がありません");
                return vec![];
            }
                    
                    info!("統合戦略から{}人の借り手を取得しました", batch.len());
                    
                    // 借り手の情報を更新
                    if let Err(e) = self.update_state().await {
                        error!("借り手情報の更新に失敗: {}", e);
                        return vec![];
                    }
                    
                    // 従来のコードと同様に清算機会を検出
                    let mut profitable_ops = Vec::new();
                    let pool_data = IPoolDataProvider::<M>::new(self.config.pool_data_provider, self.write_client.clone());
                    let pool_state = match self.get_pool_state().await {
                        Ok(ps) => ps,
            Err(e) => {
                            error!("プール状態取得エラー: {}", e);
                return vec![];
            }
        };

                    let start_time = SystemTime::now();
                    let mut tasks = Vec::new();
                    let mut borrower_count = 0;
                    
                    // バッチ内の借り手に対して並列に清算機会を評価
                    for borrower in &batch {
                        // 借り手の詳細を取得
                        let borrower_details = match self.borrowers.get(borrower) {
                            Some(details) => details.clone(),
                            None => {
                                // 借り手の情報を取得して追加
                                let pool = Pool::<M>::new(self.config.pool_address, self.write_client.clone());
                                match pool.get_user_account_data(*borrower).call().await {
                                    Ok((_, _, _, _, _, _)) => {
                                        // 借り手が存在する場合は、空の詳細で初期化
                                        let details = Borrower {
                                            address: *borrower,
                                            collateral: HashSet::new(),
                                            debt: HashSet::new(),
                                        };
                                        self.borrowers.insert(*borrower, details.clone());
                                        details
                                    },
                                    Err(e) => {
                                        warn!("借り手の情報取得に失敗: {:?} - {}", borrower, e);
                                continue;
                                    }
                                }
                            }
                        };
                        
                        // 健全性係数を取得（実際のコントラクトから）
                        let health_factor = match integrated_strategy.get_cached_health_factor(borrower).await {
                            Ok(Some(hf)) => hf,
                            _ => {
                                // キャッシュミスの場合は実際のコントラクトから取得
                                match self.get_real_health_factor(*borrower).await {
                                    Ok(real_hf) => real_hf,
                                    Err(e) => {
                                        warn!("借り手 {:?} のヘルスファクター取得エラー: {}。スキップ", borrower, e);
                                        continue; // このborrowersエントリをスキップ
                                    }
                                }
                            }
                        };
                        
                        // 1つの借り手に対して複数の清算機会を検出
                        for collateral_address in &borrower_details.collateral {
                            for debt_address in &borrower_details.debt {
                                if collateral_address.ne(debt_address) {
                                    // リクエスト制限を設ける - 並列度を管理
                                    if tasks.len() >= MAX_PARALLEL_TASKS { // 50に変更
                                        // 50件溜まったら一旦処理
                                        let processed = process_tasks(tasks).await;
                                        profitable_ops.extend(processed);
                                        tasks = Vec::new();
                                    }
                                    
                                    borrower_count += 1;
                                    let borrower_clone = *borrower;
                                    let collateral_clone = *collateral_address;
                                    let debt_clone = *debt_address;
                                    let health_factor_clone = health_factor;
                                    let pool_data_clone = pool_data.clone();
                                    let pool_state_clone = pool_state.clone();
                                    let liquidator = self.liquidator;
                                    let write_client = self.write_client.clone();
                                    let config = self.config.clone();
                                    let tokens = self.tokens.clone();
                                    let chain_id = self.chain_id;
                                    
                                    let task = tokio::spawn(run_liquidation_task(
                                        borrower_clone,
                                        collateral_clone,
                                        debt_clone,
                                        health_factor_clone,
                                        pool_data_clone,
                                        pool_state_clone,
                                        write_client,
                                        liquidator,
                                        config,
                                        tokens,
                                        chain_id
                                    ));
                                    
                                    tasks.push(task);
                                }
                            }
                        }
                    }
                    
                    // 残りのタスクを処理
                    if !tasks.is_empty() {
                        let processed = process_tasks(tasks).await;
                        profitable_ops.extend(processed);
                    }
                    
                    let end_time = SystemTime::now();
                    let duration = end_time.duration_since(start_time).unwrap();
                    info!("清算機会の検出完了: {}件の借り手を{}ミリ秒で処理", borrower_count, duration.as_millis());
                    
                    // 清算機会をActionに変換
                    let mut actions = Vec::new();
                    for op in profitable_ops {
                        match self.build_liquidation(&op).await {
                            Ok(tx) => {
                                let gas_bid_info = GasBidInfo {
                                    total_profit: match U256::from_dec_str(&op.profit_usd.abs().to_string()) {
                                        Ok(profit) => profit,
                                        Err(e) => {
                                            error!("利益の変換に失敗: {}", e);
                                            U256::zero()
                                        }
                                    },
                                    bid_percentage: self.bid_percentage,
                                };
                                
                                actions.push(Action::SubmitTx(SubmitTxToMempool {
                                    tx,
                                    gas_bid_info: Some(gas_bid_info),
                                }));
                            }
                            Err(e) => {
                                error!("清算トランザクション作成エラー: {}", e);
                            }
                        }
                    }
                    
                    actions
                        }
                Err(e) => {
                    error!("統合戦略からのバッチ取得エラー: {}", e);
                    vec![]
                }
            }
        } else {
            // 統合戦略が無効な場合は従来の処理を実行
            vec![]
        }
    }
    
    async fn update_state(&mut self) -> Result<()> {
        let current_block = self.archive_client.get_block_number().await?;
        let current_block_num = current_block.as_u64();
        
        // 既に処理済みのブロックはスキップ
        if current_block_num <= self.last_block_number {
            return Ok(());
        }
        
        // 初回スキャンかどうかを判定
        if !self.initial_scan_completed {
            info!("🔄 初回スキャン: アーカイブRPCを使用して履歴データを取得");
            return self.perform_initial_scan(current_block_num).await;
        }
        
        // 🔄 修正: 前回スキャンから最新ブロックまでのすべてのブロックをスキャン
        let from_block = self.last_block_number + 1;
        let to_block = current_block_num;
        
        if from_block > to_block {
            return Ok(());
        }
        
        let block_count = to_block - from_block + 1;
        info!("🔄 リアルタイムスキャン: ブロック {} から {} まで ({} ブロック) を処理中", 
              from_block, to_block, block_count);
        
        let start_time = std::time::SystemTime::now();
        
        // 複数ブロックのログを並列取得
        let (borrow_logs_result, supply_logs_result) = tokio::join!(
            self.get_borrow_logs(U64::from(from_block), U64::from(to_block)),
            self.get_supply_logs(U64::from(from_block), U64::from(to_block))
        );
        
        let borrow_logs = borrow_logs_result?;
        let supply_logs = supply_logs_result?;
        
        let fetch_duration = start_time.elapsed().unwrap();
        
        // ログの処理
        let mut new_borrowers = 0;
        let mut updated_borrowers = 0;
        
        for log in borrow_logs.iter() {
            let user = log.on_behalf_of;
            if let Some(borrower) = self.borrowers.get_mut(&user) {
                borrower.debt.insert(log.reserve);
                updated_borrowers += 1;
            } else {
                self.borrowers.insert(
                    user,
                    Borrower {
                        address: user,
                        collateral: HashSet::new(),
                        debt: HashSet::from([log.reserve]),
                    },
                );
                new_borrowers += 1;
            }
        }

        for log in supply_logs.iter() {
            let user = log.on_behalf_of;
            if let Some(borrower) = self.borrowers.get_mut(&user) {
                borrower.collateral.insert(log.reserve);
                updated_borrowers += 1;
            } else {
                self.borrowers.insert(
                    user,
                    Borrower {
                        address: user,
                        collateral: HashSet::from([log.reserve]),
                        debt: HashSet::new(),
                    },
                );
                new_borrowers += 1;
            }
        }

        self.last_block_number = to_block;
        
        info!(
            "⚡ ブロック {} - {} 処理完了: {}ms | 新規借り手: {} | 更新借り手: {} | ログ: {}+{}",
            from_block,
            to_block,
            fetch_duration.as_millis(),
            new_borrowers,
            updated_borrowers,
            borrow_logs.len(),
            supply_logs.len()
        );
        
        Ok(())
    }

    // 初回スキャン専用メソッド（アーカイブRPC使用、5000ブロックずつ処理）
    async fn perform_initial_scan(&mut self, current_block_num: u64) -> Result<()> {
        let from_block = self.last_block_number + 1;
        let to_block = current_block_num;

        if from_block >= to_block {
            self.initial_scan_completed = true;
            return Ok(());
        }

        info!("📚 初回履歴スキャン開始: ブロック {} から {} まで ({} ブロック) - 5000ブロックずつ効率処理", 
              from_block, to_block, to_block - from_block + 1);

        // キャッシュの読み込み
        match self.load_cache().await {
            Ok(cache) => {
                info!("💾 キャッシュの読み込みに成功");
                self.borrowers = cache.borrowers;
                // キャッシュから読み込んだ場合は、その後のブロックから開始
                if cache.last_block_number > self.last_block_number {
                    self.last_block_number = cache.last_block_number;
                }
            }
            Err(e) => {
                info!("💾 キャッシュの読み込みに失敗: {}、新規作成を開始", e);
                self.borrowers = HashMap::new();
            }
        }

        // 再計算（キャッシュ読み込み後）
        let from_block = self.last_block_number + 1;
        if from_block >= to_block {
            self.initial_scan_completed = true;
            return Ok(());
        }

        let total_blocks = to_block - from_block + 1;
        info!("📊 初回スキャン範囲確定: {} ブロック（{} から {}）", total_blocks, from_block, to_block);
        
        let start_time = std::time::SystemTime::now();

        // 並列でログを取得（5000ブロックずつ効率処理）
        info!("🚀 並列ログ取得開始（5000ブロック/チャンク）");
        let (borrow_logs_result, supply_logs_result) = tokio::join!(
            self.get_initial_scan_borrow_logs(from_block, to_block),
            self.get_initial_scan_supply_logs(from_block, to_block)
        );

        let borrow_logs = borrow_logs_result?;
        let supply_logs = supply_logs_result?;
        
        let fetch_time = start_time.elapsed().unwrap();
        info!("⚡ ログ取得完了: {}秒 | 借入ログ: {}件 | 供給ログ: {}件", 
              fetch_time.as_secs(), borrow_logs.len(), supply_logs.len());

        // ログの処理（効率化）
        let process_start = std::time::SystemTime::now();
        let mut new_borrowers = 0;
        let mut updated_borrowers = 0;
        
        // 借入ログの処理
        for log in borrow_logs {
            let user = log.on_behalf_of;
            if let Some(borrower) = self.borrowers.get_mut(&user) {
                borrower.debt.insert(log.reserve);
                updated_borrowers += 1;
            } else {
                self.borrowers.insert(
                    user,
                    Borrower {
                        address: user,
                        collateral: HashSet::new(),
                        debt: HashSet::from([log.reserve]),
                    },
                );
                new_borrowers += 1;
            }
        }

        // 供給ログの処理
        for log in supply_logs {
            let user = log.on_behalf_of;
            if let Some(borrower) = self.borrowers.get_mut(&user) {
                borrower.collateral.insert(log.reserve);
                updated_borrowers += 1;
            } else {
                self.borrowers.insert(
                    user,
                    Borrower {
                        address: user,
                        collateral: HashSet::from([log.reserve]),
                        debt: HashSet::new(),
                    },
                );
                new_borrowers += 1;
            }
        }

        let process_time = process_start.elapsed().unwrap();
        let total_time = start_time.elapsed().unwrap();
        
        self.last_block_number = to_block;
        self.initial_scan_completed = true; // 初回スキャン完了
        
        info!("🎉 初回スキャン完了!");
        info!("   📊 処理統計:");
        info!("      ⏱️  総時間: {:.2}秒", total_time.as_secs_f64());
        info!("      📥 ログ取得: {:.2}秒", fetch_time.as_secs_f64());
        info!("      🔄 ログ処理: {:.2}秒", process_time.as_secs_f64());
        info!("      📈 処理速度: {:.0} ブロック/秒", total_blocks as f64 / total_time.as_secs_f64());
        info!("      👥 新規借り手: {}", new_borrowers);
        info!("      🔄 更新借り手: {}", updated_borrowers);
        info!("      🏦 総借り手数: {}", self.borrowers.len());
        info!("🔄 リアルタイムスキャンモードに切り替えます");
        
        Ok(())
    }

    async fn approve_tokens(&mut self) -> Result<()> {
        let liquidator = Liquidator::new(self.liquidator, self.write_client.clone());

        let mut nonce = self
            .write_client
            .get_transaction_count(
                self.write_client
                    .default_sender()
                    .ok_or(anyhow!("No connected sender"))?,
                None,
            )
            .await?;
        for token_address in self.tokens.keys() {
            let token = IERC20::new(token_address.clone(), self.write_client.clone());
            let allowance = token
                .allowance(self.liquidator, self.config.pool_address)
                .call()
                .await?;
            info!("approve token: {:?}", token_address);
            info!("allowance: {:?}", allowance);
            if allowance == U256::zero() {
                liquidator
                    .approve_pool(*token_address)
                    .nonce(nonce)
                    .send()
                    .await
                    .map_err(|e| {
                        error!("approve failed: {:?}", e);
                        e
                    })?;
                nonce = nonce + 1;
            }
        }

        Ok(())
    }

    async fn update_token_configs(&mut self) -> Result<()> {
        info!("Updating token configs");
        let pool_data = IPoolDataProvider::<M>::new(self.config.pool_data_provider, self.archive_client.clone());
        
        // 必要なトークンのみを取得
        let essential_tokens = vec![
            "WHYPE", "wstHYPE", "UBTC", "UETH", "USDe", "USD₮0"
        ];
        
        for symbol in essential_tokens {
            let token_address = match symbol {
                "WHYPE" => "0x5555555555555555555555555555555555555555",
                "wstHYPE" => "0x94e8396e0869c9f2200760af0621afd240e1cf38",
                "UBTC" => "0x9fdbda0a5e284c32744d2f17ee5c74b284993463",
                "UETH" => "0xbe6727b535545c67d5caa73dea54865b92cf7907",
                "USDe" => "0x5d3a1ff2b6bab83b63cd9ad0787074081a52ef34",
                "USD₮0" => "0xb8ce59fc3717ada4c02eadf9682a9e934f625ebb",
                _ => continue,
            };
            
            let address = Address::from_str(token_address)?;
            
            // 必要な情報のみを取得
            let (decimals, ltv, threshold, bonus, reserve, _, _, _, _, _) = pool_data
                .get_reserve_configuration_data(address)
                .await?;
                
            let protocol_fee = pool_data
                .get_liquidation_protocol_fee(address)
                .await?;
                
            // aトークンアドレスを取得
            let a_token_address = match symbol {
                "WHYPE" => "0x5555555555555555555555555555555555555555",
                "wstHYPE" => "0x94e8396e0869c9f2200760af0621afd240e1cf38",
                "UBTC" => "0x9fdbda0a5e284c32744d2f17ee5c74b284993463",
                "UETH" => "0xbe6727b535545c67d5caa73dea54865b92cf7907",
                "USDe" => "0x5d3a1ff2b6bab83b63cd9ad0787074081a52ef34",
                "USD₮0" => "0xb8ce59fc3717ada4c02eadf9682a9e934f625ebb",
                _ => continue,
            };
            
            self.tokens.insert(
                address,
                TokenConfig {
                    address,
                    a_address: Address::from_str(a_token_address)?,
                    decimals: decimals.low_u64(),
                    ltv: ltv.low_u64(),
                    liquidation_threshold: threshold.low_u64(),
                    liquidation_bonus: bonus.low_u64(),
                    reserve_factor: reserve.low_u64(),
                    protocol_fee: protocol_fee.low_u64(),
                },
            );
            
            info!("トークン設定を更新: {}", symbol);
        }

        Ok(())
    }

    async fn load_cache(&mut self) -> Result<StateCache> {
        match File::open(self.config.state_cache_file.clone()) {
            Ok(file) => {
                info!("キャッシュファイルが存在します");
                let cache: StateCache = serde_json::from_reader(file)?;
                Ok(cache)
            }
            Err(_) => {
                info!("キャッシュファイルが見つかりません。新規作成します");
                Ok(StateCache::new(
                    self.config.creation_block,
                    HashMap::new()
                ))
            }
        }
    }

    fn write_intermediate_cache(&self, block_number: u64) {
        // キャッシュの書き込みを最小限に抑える
        let cache = StateCache {
            last_block_number: block_number,
            borrowers: HashMap::new(), // 空のHashMapを使用
        };
        
        if let Err(e) = File::create(self.config.state_cache_file.clone())
            .and_then(|mut file| file.write_all(serde_json::to_string(&cache)?.as_bytes()))
        {
            error!("キャッシュの書き込みに失敗: {}", e);
        }
    }

    // 水没借り手と危険ゾーン借り手の取得
    async fn get_underwater_and_at_risk_borrowers(&mut self) -> Result<Vec<(Address, U256)>> {
        let pool = Pool::<M>::new(self.config.pool_address, self.write_client.clone());
        let underwater_borrowers = self.scan_all_tiers(&pool).await?;
        
        for (borrower, health_factor) in &underwater_borrowers {
            self.notify_underwater_borrower(*borrower, *health_factor).await;
        }
        
        Ok(underwater_borrowers)
    }
    
    // 階層を更新するメソッド
    async fn update_tiers(&mut self) -> Result<()> {
        info!("Updating borrower tiers");
        let now = SystemTime::now();
        
        // 前回の更新から15分以上経過している場合のみ更新
        if let Some(last_update) = self.scan_state.last_tier_update {
            if now.duration_since(last_update).unwrap().as_secs() < 900 { // 15分 = 900秒
                info!("Skipping tier update, last update was less than 15 minutes ago");
                return Ok(());
            }
        }
        
        // 全借り手の健全性係数を取得
        let pool = Pool::<M>::new(self.config.pool_address, self.write_client.clone());
        let mut borrower_health_factors = HashMap::new();
        
        // すべての借り手を小さなチャンクに分割して処理
        let borrowers: Vec<Address> = self.borrowers.keys().cloned().collect();
        for chunk in borrowers.chunks(MULTICALL_CHUNK_SIZE) {
        let mut multicall = Multicall::new(
            self.write_client.clone(),
            Some(self.config.multicall3_address.into()), 
            ).await?;

            for borrower in chunk {
                multicall.add_call(pool.get_user_account_data(*borrower), false);
            }
            
            let results: Vec<(U256, U256, U256, U256, U256, U256)> = multicall.call_array().await?;
            
            for (i, (_, _, _, _, _, health_factor)) in results.iter().enumerate() {
                borrower_health_factors.insert(chunk[i], *health_factor);
            }
        }
        
        // 各階層の借り手リストをクリア
        for tier in &mut self.scan_state.tiers {
            tier.borrowers.clear();
        }
        
        // 借り手を適切な階層に割り当て
        for (borrower, health_factor) in borrower_health_factors {
            if !self.borrowers.contains_key(&borrower) || self.borrowers[&borrower].debt.is_empty() {
                continue; // 負債のない借り手はスキップ
            }
            
            // 適切な階層を見つける
            for tier in &mut self.scan_state.tiers {
                let (min, max) = tier.health_factor_range;
                if health_factor >= min && health_factor < max {
                    tier.borrowers.push(borrower);
                    break;
                }
            }
        }
        
        // 各階層の借り手数をログ出力
        for tier in &self.scan_state.tiers {
            info!("{} tier: {} borrowers", tier.name, tier.borrowers.len());
        }
        
        // 最終更新時刻を記録
        self.scan_state.last_tier_update = Some(now);
        
        Ok(())
    }
    
    // 階層別に並列スキャンを実行
    async fn scan_tier(&self, tier: &mut BorrowerTier, pool: &Pool<M>) -> Result<Vec<(Address, U256)>> {
        let now = SystemTime::now();
        
        // スキャン間隔をチェック
        if let Some(last_scan) = tier.last_scan {
            if now.duration_since(last_scan).unwrap() < tier.scan_interval {
                debug!("Skipping {} tier scan, not due yet", tier.name);
                return Ok(vec![]);
            }
        }
        
        if tier.borrowers.is_empty() {
            tier.last_scan = Some(now);
            return Ok(vec![]);
        }
        
        info!("Scanning {} tier with {} borrowers", tier.name, tier.borrowers.len());
        
        // 並列処理の設定
        let mut underwater = Vec::new();
        let parallel_factor = tier.parallel_factor.max(1);
        let chunk_size = (tier.borrowers.len() + parallel_factor - 1) / parallel_factor;
        
        if chunk_size == 0 {
            tier.last_scan = Some(now);
            return Ok(vec![]);
        }
        
        // 並列処理用のタスクを作成
        let mut tasks = Vec::new();
        let multicall_address = self.config.multicall3_address;
        
        for (i, chunk) in tier.borrowers.chunks(chunk_size).enumerate() {
            let chunk_vec = chunk.to_vec();
            let pool_clone = pool.clone();
            
            info!("Creating task {} with {} borrowers for {} tier", 
                  i, chunk_vec.len(), tier.name);
            
            let task = tokio::spawn(async move {
                let mut results = Vec::new();
                
                // 小さなチャンクに分割してMulticallを使用
                for small_chunk in chunk_vec.chunks(50) {
                    let mut multicall = match Multicall::new(
                        pool_clone.client().clone(),
                        Some(multicall_address.into()),
                    ).await {
                        Ok(m) => m,
                        Err(e) => return Err(anyhow!("Multicallの作成に失敗: {}", e)),
                    };
                    
                    for borrower in small_chunk {
                        multicall.add_call(pool_clone.get_user_account_data(*borrower), false);
                    }
                    
                    match multicall.call_array::<(U256, U256, U256, U256, U256, U256)>().await {
                        Ok(health_factors) => {
                            for (i, (_, _, _, _, _, health_factor)) in health_factors.iter().enumerate() {
                                let borrower = small_chunk[i];
                                let one_eth = U256::from_dec_str("1000000000000000000").unwrap();
                                
                                if health_factor < &one_eth {
                                    results.push((borrower, *health_factor));
                                }
                            }
                        },
                        Err(e) => return Err(anyhow!("Multicall実行エラー: {}", e)),
                    }
                }
                
                Ok(results)
            });
            
            tasks.push(task);
        }
        
        // すべてのタスクの結果を収集
        for task in futures::future::join_all(tasks).await {
            match task {
                Ok(Ok(results)) => underwater.extend(results),
                Ok(Err(e)) => warn!("Task error in {} tier: {}", tier.name, e),
                Err(e) => warn!("Join error in {} tier: {}", tier.name, e),
            }
        }
        
        // 最後のスキャン時刻を更新
        tier.last_scan = Some(now);
        
        info!("Found {} underwater borrowers in {} tier", underwater.len(), tier.name);
        Ok(underwater)
    }

    // すべての階層をスキャン
    async fn scan_all_tiers(&mut self, pool: &Pool<M>) -> Result<Vec<(Address, U256)>> {
        // 必要に応じて階層を更新
        self.update_tiers().await?;
        
        let mut all_underwater = Vec::new();
        let start_time = SystemTime::now();
        
        // 効率的な並列処理のために全階層を同時にスキャン開始する
        let mut tasks: Vec<tokio::task::JoinHandle<Result<(String, Vec<(Address, U256)>, SystemTime, usize)>>> = Vec::new();
        
        // 階層情報を事前に複製して、後のループでself.scan_state.tiersへの参照を避ける
        let mut scan_targets = Vec::new();
        
        // イテレータを使って、階層ごとにチェック
        let tiers = &mut self.scan_state.tiers;
        for i in 0..tiers.len() {
            let now = SystemTime::now();
            
            // スキャン間隔をチェック - スキャンが必要な階層のみ追加
            if let Some(last_scan) = tiers[i].last_scan {
                if now.duration_since(last_scan).unwrap() < tiers[i].scan_interval {
                    debug!("{} 階層のスキャンはまだ予定時刻ではありません", tiers[i].name);
                    continue;
                }
            }
            
            // 借り手がいない階層はスキップ
            if tiers[i].borrowers.is_empty() {
                // 最後のスキャン時刻を更新して次へ
                tiers[i].last_scan = Some(now);
                continue;
            }
            
            // 並列処理用にクローンしてスキャン対象に追加
            scan_targets.push((i, tiers[i].clone(), now));
        }
        
        // スキャン対象の階層に対して並列処理を開始
        for (tier_idx, tier_clone, _scan_time) in scan_targets {  // 未使用変数へのアンダースコア追加
            info!("{} 階層を並列スキャン開始 ({}人の借り手)", tier_clone.name, tier_clone.borrowers.len());
            
            let pool_clone = pool.clone();
            let multicall_address = self.config.multicall3_address;
            
            // 階層ごとに並列タスクを作成
            let task = tokio::spawn(async move {
                let tier_start = SystemTime::now();
                let mut underwater = Vec::new();
                
                // 借り手をチャンクに分割
                let chunk_size = (tier_clone.borrowers.len() + tier_clone.parallel_factor - 1) / 
                           tier_clone.parallel_factor.max(1);
                
                if chunk_size == 0 {
                    return Ok((tier_clone.name, underwater, tier_start, tier_idx));
                }
                
                // 各チャンクを並列処理
                let mut chunk_tasks = Vec::new();
                for chunk in tier_clone.borrowers.chunks(chunk_size) {
                    let chunk_vec = chunk.to_vec();
                    let pool_clone = pool_clone.clone();
                    
                    let chunk_task = tokio::spawn(async move {
                        let mut results = Vec::new();
                        
                        // 小さなチャンクに分割してMulticallを使用
                        for small_chunk in chunk_vec.chunks(50) {
                            let mut multicall = match Multicall::new(
                                pool_clone.client().clone(),
                                Some(multicall_address.into()),
                            ).await {
                                Ok(m) => m,
                                Err(e) => return Err(anyhow!("Multicallの作成に失敗: {}", e)),
                            };
                            
                            for borrower in small_chunk {
                                multicall.add_call(pool_clone.get_user_account_data(*borrower), false);
                            }
                            
                            match multicall.call_array::<(U256, U256, U256, U256, U256, U256)>().await {
                                Ok(health_factors) => {
                                    for (i, (_, _, _, _, _, health_factor)) in health_factors.iter().enumerate() {
                                        if i < small_chunk.len() { // 安全チェック
                                            let borrower = small_chunk[i];
                                            let one_eth = U256::from_dec_str("1000000000000000000").unwrap();
                                            
                                            // 健全性係数が1.0未満の借り手のみを追加
                                            if health_factor < &one_eth {
                                                results.push((borrower, *health_factor));
                                            }
                                        }
                                    }
                                },
                                Err(e) => return Err(anyhow!("Multicall実行エラー: {}", e)),
                            }
                        }
                        
                        Ok(results)
                    });
                    
                    chunk_tasks.push(chunk_task);
                }
                
                // すべてのチャンクタスクの結果を収集
                for chunk_result in futures::future::join_all(chunk_tasks).await {
                    match chunk_result {
                        Ok(Ok(results)) => underwater.extend(results),
                        Ok(Err(e)) => warn!("{} 階層のチャンクタスクでエラー: {}", tier_clone.name, e),
                        Err(e) => warn!("{} 階層のチャンク結合でエラー: {}", tier_clone.name, e),
                    }
                }
                
                // 結果と階層名を返す
                let duration = SystemTime::now().duration_since(tier_start)
                    .unwrap_or(Duration::from_secs(0));
                info!("{} 階層のスキャン完了。{}人の水没借り手を検出。所要時間: {:.2}秒", 
                      tier_clone.name, underwater.len(), duration.as_secs_f64());
                
                Ok((tier_clone.name, underwater, tier_start, tier_idx))
            });
            
            tasks.push(task);
        }
        
        // すべてのタスクが完了するのを待つ
        let results = futures::future::join_all(tasks).await;
        
        // 結果を処理して有効なタスク結果のみを取得
        for task_result in results.into_iter() {
            match task_result {
                Ok(Ok((tier_name, underwater, scan_time, tier_idx))) => {
                    // 結果を全体リストに追加
                    all_underwater.extend(underwater);
                    
                    // 階層の最終スキャン時刻を更新
                    if tier_idx < self.scan_state.tiers.len() {
                        self.scan_state.tiers[tier_idx].last_scan = Some(scan_time);
                    }
                    
                    // Critical/High Risk階層で水没借り手が見つかった場合のチェック
                    if (tier_idx == 0 || tier_idx == 1) && !all_underwater.is_empty() {
                        info!("{} 階層で水没借り手が検出されました。優先度の低い階層のスキャンはスキップします", tier_name);
                        // 低優先度階層の最終スキャン時刻も更新して一時的にスキップ
                        for j in (tier_idx + 1)..self.scan_state.tiers.len() {
                            // 10秒前にスキャンしたことにする（次回も確実にスキャンする）
                            let fake_time = SystemTime::now()
                                .checked_sub(Duration::from_secs(self.scan_state.tiers[j].scan_interval.as_secs() - 10))
                                .unwrap_or_else(SystemTime::now);
                            self.scan_state.tiers[j].last_scan = Some(fake_time);
                        }
                    }
                },
                Ok(Err(e)) => {
                    warn!("階層のスキャンでエラー: {}", e);
                },
                Err(e) => {
                    warn!("階層のタスク実行でエラー: {}", e);
                }
            }
        }
        
        // 結果をヘルスファクター順にソート（最も危険なものが先頭に）
        all_underwater.sort_by(|(_, a_health): &(Address, U256), (_, b_health): &(Address, U256)| a_health.cmp(b_health));
        
        let duration = SystemTime::now().duration_since(start_time)
            .unwrap_or(Duration::from_secs(0));
        info!("全階層スキャン完了。合計 {} 人の水没借り手を検出。総所要時間: {:.2}秒",
              all_underwater.len(), duration.as_secs_f64());
        
        Ok(all_underwater)
    }

    // ペンディング清算の確認メソッドを追加
    async fn check_pending_liquidations(&mut self) -> Result<()> {
        let now = SystemTime::now();
        let mut i = 0;
        while i < self.pending_liquidations.len() {
            let liquidation = &self.pending_liquidations[i];
            
            // 30分以上経過したトランザクションを確認
            if now.duration_since(liquidation.submission_time).unwrap().as_secs() > 1800 {
                match self.write_client.get_transaction_receipt(liquidation.tx_hash).await {
                    Ok(Some(receipt)) => {
                        if receipt.status == Some(U64::from(1)) {
                            info!("清算トランザクション成功: {:?}", liquidation.tx_hash);
                        } else {
                            warn!("清算トランザクション失敗: {:?}", liquidation.tx_hash);
                        }
                    }
                    Ok(None) => {
                        warn!("清算トランザクションが見つかりません: {:?}", liquidation.tx_hash);
                    }
                    Err(e) => {
                        error!("トランザクション確認エラー: {}", e);
                    }
                }
                self.pending_liquidations.remove(i);
            } else {
                i += 1;
            }
        }
        Ok(())
    }

    // プール状態の取得メソッドを追加
    async fn get_pool_state(&mut self) -> Result<PoolState> {
        let oracle = IAaveOracle::new(self.config.oracle_address, self.archive_client.clone());
        let mut prices = HashMap::new();
        
        for token_address in self.tokens.keys() {
            let price = oracle.get_asset_price(*token_address).call().await?;
            prices.insert(*token_address, price);
        }
        
        Ok(PoolState { prices })
    }

    // 借入ログの取得メソッドを追加（1ブロックずつ処理）
    async fn get_borrow_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<BorrowFilter>> {
        let mut all_logs = Vec::new();
        let start_block = from_block.as_u64();
        let end_block = to_block.as_u64();
        
        info!("ログ取得開始: ブロック {} から {} まで（{}ブロック）", 
              start_block, end_block, end_block - start_block + 1);
        
        // 1ブロックずつ処理
        for current_block in start_block..=end_block {
            info!("ブロック {} のログ取得を試行中", current_block);

            let mut retry_count = 0;
            let mut success = false;
            
            while !success && retry_count < MAX_RETRIES {
                match self.try_get_logs(current_block, current_block).await {
                    Ok(logs) => {
                        let logs_len = logs.len();
                        all_logs.extend(logs);
                        success = true;
                        if logs_len > 0 {
                            info!("ブロック {} のログ取得成功 ({} 件)", current_block, logs_len);
                        }
                    },
                    Err(e) => {
                        retry_count += 1;
                        if retry_count < MAX_RETRIES {
                            warn!(
                                "ブロック {} のログ取得エラー（試行 {}/{}）: {}。再試行します...",
                                current_block, retry_count, MAX_RETRIES, e
                            );
                            // 短い間隔でリトライ
                            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                        } else {
                            error!("ブロック {} のログ取得に失敗: {}", current_block, e);
                            // 失敗したブロックはスキップして次に進む
                        }
                    }
                }
            }
            
            // ブロック間の短い間隔
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        
        info!("ログ取得完了: 合計 {} 件のログを取得", all_logs.len());
        Ok(all_logs)
    }

    async fn get_supply_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<SupplyFilter>> {
        let mut all_logs = Vec::new();
        let start_block = from_block.as_u64();
        let end_block = to_block.as_u64();
        
        info!("供給ログ取得開始: ブロック {} から {} まで（{}ブロック）", 
              start_block, end_block, end_block - start_block + 1);
        
        // 1ブロックずつ処理
        for current_block in start_block..=end_block {
            info!("ブロック {} の供給ログ取得を試行中", current_block);

            let mut retry_count = 0;
            let mut success = false;
            
            while !success && retry_count < MAX_RETRIES {
                match self.try_get_supply_logs(current_block, current_block).await {
                    Ok(logs) => {
                        let logs_len = logs.len();
                        all_logs.extend(logs);
                        success = true;
                        if logs_len > 0 {
                            info!("ブロック {} の供給ログ取得成功 ({} 件)", current_block, logs_len);
                        }
                    },
                    Err(e) => {
                        retry_count += 1;
                        if retry_count < MAX_RETRIES {
                            warn!(
                                "ブロック {} の供給ログ取得エラー（試行 {}/{}）: {}。再試行します...",
                                current_block, retry_count, MAX_RETRIES, e
                            );
                            // 短い間隔でリトライ
                            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                        } else {
                            error!("ブロック {} の供給ログ取得に失敗: {}", current_block, e);
                            // 失敗したブロックはスキップして次に進む
                        }
                    }
                }
            }
            
            // ブロック間の短い間隔
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        
        info!("供給ログ取得完了: 合計 {} 件のログを取得", all_logs.len());
        Ok(all_logs)
    }

    // 清算トランザクションの構築メソッドを追加
    async fn build_liquidation(&self, op: &LiquidationOpportunity) -> Result<TypedTransaction> {
        let liquidator = Liquidator::new(self.liquidator, self.write_client.clone());
        let usdt0_address = Address::from_str(Self::USDT0_ADDRESS)?;
        
        // フォールバック戦略付きパス作成
        let swap_path = create_swap_path_with_ioa_protection(op.collateral, op.debt, &self.config.default_liq_path)?;
        
        let tx = liquidator.liquidate(
            op.collateral,
            op.debt,
            op.borrower,
            op.debt_to_cover,
            Bytes::from(swap_path),
            self.config.default_liq_path.clone(),
            usdt0_address,
        );
    
        Ok(tx.tx)
    }

    async fn try_backup_rpc_supply_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<SupplyFilter>> {
        // バックアップRPCエンドポイントの設定
        let backup_rpc_url = std::env::var("BACKUP_RPC_URL")
            .unwrap_or_else(|_| "https://rpc.hyperlend.finance/archive".to_string());
        
        let provider = match Provider::<ethers::providers::Http>::try_connect(&backup_rpc_url).await {
            Ok(p) => p,
            Err(e) => {
                error!("バックアップRPCへの接続に失敗: {}", e);
                return Err(anyhow!("バックアップRPCへの接続に失敗"));
            }
        };

        let pool = Pool::<ethers::providers::Provider<ethers::providers::Http>>::new(
            self.config.pool_address,
            Arc::new(provider)
        );

        let filter = pool.supply_filter()
            .from_block(from_block)
            .to_block(to_block);
        
        let timeout = tokio::time::Duration::from_secs(5); // タイムアウトを5秒に短縮
        
        match tokio::time::timeout(timeout, filter.query()).await {
            Ok(result) => match result {
                Ok(logs) => {
                    info!("バックアップRPCでブロック範囲 {} から {} の供給ログ取得成功 ({} 件)", 
                          from_block, to_block, logs.len());
                    Ok(logs)
                },
                Err(e) => {
                    error!("バックアップRPCでの供給ログ取得に失敗: {}", e);
                    Err(anyhow!("バックアップRPCでの供給ログ取得に失敗: {}", e))
                }
            },
            Err(_) => {
                error!("バックアップRPCでの供給ログ取得がタイムアウト");
                Err(anyhow!("バックアップRPCでの供給ログ取得がタイムアウト"))
            }
        }
    }

    async fn try_backup_rpc_borrow_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<BorrowFilter>> {
        // バックアップRPCエンドポイントの設定
        let backup_rpc_url = std::env::var("BACKUP_RPC_URL")
            .unwrap_or_else(|_| "https://rpc.hyperliquid.xyz/evm".to_string());
        
        let provider = match Provider::<ethers::providers::Http>::try_connect(&backup_rpc_url).await {
            Ok(p) => p,
            Err(e) => {
                error!("バックアップRPCへの接続に失敗: {}", e);
                return Err(anyhow!("バックアップRPCへの接続に失敗"));
            }
        };

        let pool = Pool::<ethers::providers::Provider<ethers::providers::Http>>::new(
            self.config.pool_address,
            Arc::new(provider)
        );

        let filter = pool.borrow_filter()
            .from_block(from_block)
            .to_block(to_block);
        
        let timeout = tokio::time::Duration::from_secs(10);
        
        // 最大3回のリトライ
        let mut retry_count = 0;
        let max_retries = 3;
        
        while retry_count < max_retries {
            match tokio::time::timeout(timeout, filter.query()).await {
                Ok(result) => match result {
                    Ok(logs) => {
                        info!("バックアップRPCでブロック範囲 {} から {} の借入ログ取得成功 ({} 件)", 
                              from_block, to_block, logs.len());
                        return Ok(logs);
                    },
                    Err(e) => {
                        retry_count += 1;
                        warn!("バックアップRPCでの借入ログ取得に失敗 (試行 {}/{}): {}", 
                              retry_count, max_retries, e);
                        
                        if retry_count < max_retries {
                            tokio::time::sleep(
                                tokio::time::Duration::from_secs(2u64.pow(retry_count))
                            ).await;
                            continue;
                        }
                        return Err(anyhow!("バックアップRPCでの借入ログ取得に失敗: {}", e));
                    }
                },
                Err(_) => {
                    retry_count += 1;
                    warn!("バックアップRPCでの借入ログ取得がタイムアウト (試行 {}/{})", 
                          retry_count, max_retries);
                    
                    if retry_count < max_retries {
                        tokio::time::sleep(
                            tokio::time::Duration::from_secs(2u64.pow(retry_count))
                        ).await;
                        continue;
                    }
                    return Err(anyhow!("バックアップRPCでの借入ログ取得がタイムアウト"));
                }
            }
        }
        
        Err(anyhow!("バックアップRPCでの借入ログ取得が最大リトライ回数を超えました"))
    }

    // Hyperlend Archive RPCを使用して借入ログを取得
    async fn try_hyperlend_archive_rpc_borrow_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<BorrowFilter>> {
        let hyperlend_archive_rpc = "https://rpc.hyperlend.finance/archive";
        
        let provider = match Provider::<ethers::providers::Http>::try_connect(hyperlend_archive_rpc).await {
            Ok(p) => p,
            Err(e) => {
                error!("Hyperlend Archive RPCへの接続に失敗: {}", e);
                return Err(anyhow!("Hyperlend Archive RPCへの接続に失敗"));
            }
        };

        let pool = Pool::<ethers::providers::Provider<ethers::providers::Http>>::new(
            self.config.pool_address,
            Arc::new(provider)
        );

        let filter = pool.borrow_filter()
            .from_block(from_block)
            .to_block(to_block);
        
        let timeout = tokio::time::Duration::from_secs(20); // タイムアウトを20秒に延長
        
        match tokio::time::timeout(timeout, filter.query()).await {
            Ok(result) => match result {
                Ok(logs) => Ok(logs),
                Err(e) => Err(anyhow!("Hyperlend Archive RPCでの借入ログ取得に失敗: {}", e))
            },
            Err(_) => Err(anyhow!("Hyperlend Archive RPCでの借入ログ取得がタイムアウト"))
        }
    }

    // Hyperliquid RPCを使用して借入ログを取得
    async fn try_hyperliquid_rpc_borrow_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<BorrowFilter>> {
        let hyperliquid_rpc = "https://rpc.hyperliquid.xyz/evm";
        
        let provider = match Provider::<ethers::providers::Http>::try_connect(hyperliquid_rpc).await {
            Ok(p) => p,
            Err(e) => {
                error!("Hyperliquid RPCへの接続に失敗: {}", e);
                return Err(anyhow!("Hyperliquid RPCへの接続に失敗"));
            }
        };

        let pool = Pool::<ethers::providers::Provider<ethers::providers::Http>>::new(
            self.config.pool_address,
            Arc::new(provider)
        );

        let filter = pool.borrow_filter()
            .from_block(from_block)
            .to_block(to_block);
        
        let timeout = tokio::time::Duration::from_secs(20); // タイムアウトを20秒に延長
        
        match tokio::time::timeout(timeout, filter.query()).await {
            Ok(result) => match result {
                Ok(logs) => Ok(logs),
                Err(e) => Err(anyhow!("Hyperliquid RPCでの借入ログ取得に失敗: {}", e))
            },
            Err(_) => Err(anyhow!("Hyperliquid RPCでの借入ログ取得がタイムアウト"))
        }
    }

    // ネットワーク負荷を取得するメソッドを追加
    async fn get_network_load(&self) -> Result<f64> {
        // 最近のブロックのガス使用率を取得
        let latest_block = self.archive_client.get_block_number().await?;
        let mut total_gas_used = 0u64;
        let mut total_gas_limit = 0u64;
        let sample_size = 10;

        for i in 0..sample_size {
            let block_number = latest_block.as_u64() - i;
            if let Ok(Some(block)) = self.archive_client.get_block(block_number).await {
                total_gas_used += block.gas_used.as_u64();
                total_gas_limit += block.gas_limit.as_u64();
            }
        }

        if total_gas_limit == 0 {
            return Ok(0.0);
        }

        Ok(total_gas_used as f64 / total_gas_limit as f64)
    }

    async fn try_get_logs(&self, from_block: u64, to_block: u64) -> Result<Vec<BorrowFilter>> {
        let pool = Pool::<M>::new(self.config.pool_address, self.archive_client.clone());
        let filter = pool.borrow_filter()
            .from_block(U64::from(from_block))
            .to_block(U64::from(to_block));
        
        match tokio::time::timeout(
            tokio::time::Duration::from_secs(MAIN_RPC_TIMEOUT),
            filter.query()
        ).await {
            Ok(result) => result.map_err(|e| anyhow!("ログ取得エラー: {}", e)),
            Err(_) => Err(anyhow!("ログ取得タイムアウト")),
        }
    }

    async fn try_get_supply_logs(&self, from_block: u64, to_block: u64) -> Result<Vec<SupplyFilter>> {
        let pool = Pool::<M>::new(self.config.pool_address, self.archive_client.clone());
        let filter = pool.supply_filter()
            .from_block(U64::from(from_block))
            .to_block(U64::from(to_block));
        
        match tokio::time::timeout(
            tokio::time::Duration::from_secs(MAIN_RPC_TIMEOUT),
            filter.query()
        ).await {
            Ok(result) => result.map_err(|e| anyhow!("ログ取得エラー: {}", e)),
            Err(_) => Err(anyhow!("ログ取得タイムアウト")),
        }
    }

    // 最新ブロックの借入ログを高速取得（競合対策）
    async fn get_latest_block_borrow_logs(&mut self, block_number: u64) -> Result<Vec<BorrowFilter>> {
        // フォールバック判定: アーカイブRPCを優先使用する場合
        if let Some(_fallback_client) = self.get_fallback_archive_client() {
            info!("📚 アーカイブRPCフォールバック使用: ブロック {} の借入ログ取得", block_number);
            return self.get_borrow_logs_with_archive_fallback(block_number).await;
        }

        // リアルタイムクライアントが利用可能かチェック
        let client = if let Some(ref realtime_client) = self.realtime_client {
            info!("🚀 リアルタイムRPC使用: ブロック {} の借入ログ取得（1秒ブロック対応）", block_number);
            realtime_client.clone()
        } else {
            warn!("⚠️ リアルタイムRPC不可。アーカイブRPCにフォールバック");
            return self.get_fallback_borrow_logs(block_number).await;
        };

        let pool = Pool::<Provider<ethers::providers::Http>>::new(self.config.pool_address, client);
        let filter = pool.borrow_filter()
            .from_block(U64::from(block_number))
            .to_block(U64::from(block_number));
        
        // リアルタイムRPCは超短いタイムアウトで高速処理（1秒ブロック対応）
        match tokio::time::timeout(
            tokio::time::Duration::from_millis(800), // 2秒 → 800msに短縮（1秒ブロック対応）
            filter.query()
        ).await {
            Ok(result) => {
                match result {
                    Ok(logs) => {
                        // 成功した場合のエラーカウンターリセット
                        self.record_rpc_success();
                        if !logs.is_empty() {
                            info!("⚡ ブロック {} の借入ログ: {}件 (リアルタイムRPC, 1秒ブロック)", block_number, logs.len());
                        }
                        Ok(logs)
                    },
                    Err(e) => {
                        // エラーを記録
                        let error_msg = format!("ブロック {} の借入ログ取得エラー (リアルタイムRPC): {}", block_number, e);
                        self.record_rpc_error(&error_msg);
                        warn!("{}。フォールバック", error_msg);
                        self.get_fallback_borrow_logs(block_number).await
                    }
                }
            },
            Err(_) => {
                // タイムアウトエラーを記録
                let error_msg = format!("ブロック {} の借入ログ取得タイムアウト (リアルタイムRPC)", block_number);
                self.record_rpc_error(&error_msg);
                warn!("{}。フォールバック", error_msg);
                self.get_fallback_borrow_logs(block_number).await
            }
        }
    }

    // 最新ブロックの供給ログを高速取得（競合対策）
    async fn get_latest_block_supply_logs(&mut self, block_number: u64) -> Result<Vec<SupplyFilter>> {
        // フォールバック判定: アーカイブRPCを優先使用する場合
        if let Some(_fallback_client) = self.get_fallback_archive_client() {
            info!("📚 アーカイブRPCフォールバック使用: ブロック {} の供給ログ取得", block_number);
            return self.get_supply_logs_with_archive_fallback(block_number).await;
        }

        // リアルタイムクライアントが利用可能かチェック
        let client = if let Some(ref realtime_client) = self.realtime_client {
            info!("🚀 リアルタイムRPC使用: ブロック {} の供給ログ取得（1秒ブロック対応）", block_number);
            realtime_client.clone()
        } else {
            warn!("⚠️ リアルタイムRPC不可。アーカイブRPCにフォールバック");
            return self.get_fallback_supply_logs(block_number).await;
        };

        let pool = Pool::<Provider<ethers::providers::Http>>::new(self.config.pool_address, client);
        let filter = pool.supply_filter()
            .from_block(U64::from(block_number))
            .to_block(U64::from(block_number));
        
        // リアルタイムRPCは超短いタイムアウトで高速処理（1秒ブロック対応）
        match tokio::time::timeout(
            tokio::time::Duration::from_millis(800), // 2秒 → 800msに短縮（1秒ブロック対応）
            filter.query()
        ).await {
            Ok(result) => {
                match result {
                    Ok(logs) => {
                        // 成功した場合のエラーカウンターリセット
                        self.record_rpc_success();
                        if !logs.is_empty() {
                            info!("⚡ ブロック {} の供給ログ: {}件 (リアルタイムRPC, 1秒ブロック)", block_number, logs.len());
                        }
                        Ok(logs)
                    },
                    Err(e) => {
                        // エラーを記録
                        let error_msg = format!("ブロック {} の供給ログ取得エラー (リアルタイムRPC): {}", block_number, e);
                        self.record_rpc_error(&error_msg);
                        warn!("{}。フォールバック", error_msg);
                        self.get_fallback_supply_logs(block_number).await
                    }
                }
            },
            Err(_) => {
                // タイムアウトエラーを記録
                let error_msg = format!("ブロック {} の供給ログ取得タイムアウト (リアルタイムRPC)", block_number);
                self.record_rpc_error(&error_msg);
                warn!("{}。フォールバック", error_msg);
                self.get_fallback_supply_logs(block_number).await
            }
        }
    }

    // フォールバック用借入ログ取得（アーカイブRPC使用）
    async fn get_fallback_borrow_logs(&self, block_number: u64) -> Result<Vec<BorrowFilter>> {
        let pool = Pool::<M>::new(self.config.pool_address, self.archive_client.clone());
        let filter = pool.borrow_filter()
            .from_block(U64::from(block_number))
            .to_block(U64::from(block_number));
        
        match tokio::time::timeout(
            tokio::time::Duration::from_secs(5), // フォールバックは少し長めのタイムアウト
            filter.query()
        ).await {
            Ok(result) => {
                match result {
                    Ok(logs) => {
                        if !logs.is_empty() {
                            info!("📚 ブロック {} の借入ログ: {}件 (アーカイブRPC)", block_number, logs.len());
                        }
                        Ok(logs)
                    },
                    Err(e) => {
                        warn!("ブロック {} の借入ログ取得エラー (アーカイブRPC): {}", block_number, e);
                        Ok(vec![]) // エラー時は空のベクターを返す
                    }
                }
            },
            Err(_) => {
                warn!("ブロック {} の借入ログ取得タイムアウト (アーカイブRPC)", block_number);
                Ok(vec![]) // タイムアウト時も空のベクターを返す
            }
        }
    }

    // フォールバック用供給ログ取得（アーカイブRPC使用）
    async fn get_fallback_supply_logs(&self, block_number: u64) -> Result<Vec<SupplyFilter>> {
        warn!("🔄 フォールバック処理中: 供給ログを取得（ブロック {}）", block_number);
        
        let pool = Pool::<M>::new(self.config.pool_address, self.archive_client.clone());
        let filter = pool.supply_filter()
            .from_block(U64::from(block_number))
            .to_block(U64::from(block_number));
        
        match tokio::time::timeout(
            Duration::from_secs(MAIN_RPC_TIMEOUT),
            filter.query()
        ).await {
            Ok(result) => {
                let logs = result.map_err(|e| anyhow!("フォールバック供給ログ取得エラー: {}", e))?;
                info!("✅ フォールバック供給ログ取得成功: {}件", logs.len());
                Ok(logs)
            },
            Err(_) => {
                error!("🚨 フォールバック供給ログ取得タイムアウト");
                Ok(vec![])
            }
        }
    }


// パーセンテージ計算のヘルパー関数を追加
fn percent_mul(value: U256, percentage: u64) -> U256 {
    value * U256::from(percentage) / U256::from(10000)
}

const DISCORD_WEBHOOK_URL: &str = "https://canary.discord.com/api/webhooks/1378380473281151007/OkAPTUr0L8kNys97-WEDlIpsfgiCVuPRbFiGFrFsQgtIkYAx5c0ybYdgmpfBrAW-b1v5";

    // Discordに通知を送信する関数
    async fn send_discord_notification(&self, message: &str) {
        let client = Client::new();
        let payload = json!({
            "content": message
        });

        if let Err(e) = client
            .post(DISCORD_WEBHOOK_URL)
            .json(&payload)
            .send()
            .await
        {
            error!("Discord通知の送信に失敗: {}", e);
        }
    }

    // 水没借り手を発見したときの通知
    async fn notify_underwater_borrower(&self, borrower: Address, health_factor: U256) {
        let message = format!(
            "🚨 水没借り手を発見しました！\n借り手アドレス: {}\nヘルスファクター: {}",
            borrower,
            health_factor
        );
        self.send_discord_notification(&message).await;
    }

    // トランザクション送信判定時の通知
    async fn notify_transaction_decision(&self, borrower: Address, health_factor: U256, decision: &str) {
        let message = format!(
            "⚖️ トランザクション送信判定\n借り手アドレス: {}\nヘルスファクター: {}\n判定結果: {}",
            borrower,
            health_factor,
            decision
        );
        self.send_discord_notification(&message).await;
    }

    // トランザクション送信時の通知
    async fn notify_transaction_sent(&self, borrower: Address, tx_hash: H256) {
        let message = format!(
            "✅ トランザクションを送信しました\n借り手アドレス: {}\nトランザクションハッシュ: {}",
            borrower,
            tx_hash
        );
        self.send_discord_notification(&message).await;
    }

    // トランザクション送信判定を修正
    async fn should_send_transaction(&self, borrower: Address, health_factor: U256) -> bool {
        let decision = if health_factor < self.config.min_health_factor {
            "送信する"
        } else {
            "送信しない"
        };
        
        self.notify_transaction_decision(borrower, health_factor, decision).await;
        
        health_factor < self.config.min_health_factor
    }

    // トランザクション送信を修正
    async fn send_transaction(&self, borrower: Address, tx: TransactionRequest) -> Result<H256> {
        let tx_hash = self.write_client.send_transaction(tx, None).await?.tx_hash();
        self.notify_transaction_sent(borrower, tx_hash).await;
        Ok(tx_hash)
    }

    // 高速化モードを有効化するメソッド（1秒ブロック対応）
    pub fn enable_turbo_mode(&mut self) {
        info!("🚀 高速モードを有効化しています...（1秒ブロック対応）");
        
        // スキャン間隔をさらに短縮（1秒ブロック対応）
        self.scan_state.base_interval = 1;     // 5秒 → 1秒（ブロック毎）
        self.scan_state.reduced_interval = 1;  // 2秒 → 1秒（最速）
        self.scan_state.current_interval = 1;  // 5秒 → 1秒（ブロック毎）
        
        // 実行設定の最適化（1秒ブロック対応）
        self.execution_config.max_concurrent_txs = 20;     // 15 → 20に増加
        self.execution_config.gas_price_multiplier = 120;  // さらに積極的（130 → 120）
        self.execution_config.min_profit_multiplier = 105; // さらに積極的（110 → 105）
        self.execution_config.adjustment_interval = Duration::from_secs(60); // 120秒 → 60秒に短縮
        
        // 階層スキャン間隔の短縮（1秒ブロック対応）
        for tier in &mut self.scan_state.tiers {
            // 最小間隔を500msに設定
            let new_interval = Duration::from_millis(std::cmp::max(
                tier.scan_interval.as_millis() as u64 / 3, // 1/3に短縮
                500 // 最小500ms
            ));
            tier.scan_interval = new_interval;
            tier.parallel_factor = tier.parallel_factor * 2; // 並列度2倍
        }
        
        // バックプレッシャー閾値の調整（1秒ブロック対応）
        self.backpressure_threshold = 30; // 20 → 30に増加（高速処理対応）
        
        info!("🚀 高速モードが有効化されました（1秒ブロック対応）");
        info!("   - 基本スキャン間隔: {}秒（ブロック毎）", self.scan_state.base_interval);
        info!("   - 最大同時TX数: {}", self.execution_config.max_concurrent_txs);
        info!("   - 階層スキャン間隔: 最低500ms（超高速）");
        info!("   - 並列処理度: 2倍（1秒ブロック対応）");
        info!("   - タイムアウト: 800ms（超高速応答）");
    }
    
    // 統計情報を表示するメソッド
    pub fn print_performance_stats(&self) {
        info!("📊 パフォーマンス統計:");
        info!("   借り手総数: {}", self.borrowers.len());
        info!("   アットリスク借り手: {}", self.at_risk_borrowers.len());
        info!("   ペンディング清算: {}", self.pending_liquidations.len());
        info!("   現在のスキャン間隔: {}秒", self.scan_state.current_interval);
        info!("   バックプレッシャー状態: {}", self.is_backpressure_active);
        
        for tier in &self.scan_state.tiers {
            info!("   {} 階層: {}人 (並列度: {})", 
                  tier.name, tier.borrowers.len(), tier.parallel_factor);
        }
    }

    // 実験的リアルタイムRPCクライアントの初期化
    pub async fn init_realtime_client(&mut self) -> Result<()> {
        info!("🚀 実験的リアルタイムRPCクライアントの初期化を開始（1秒ブロック対応）");
        
        // 環境に応じたリアルタイムRPCのURL決定
        let realtime_rpc_url = std::env::var("REALTIME_RPC_URL").unwrap_or_else(|_| {
            // write_clientから環境判定（main.rsと同じロジック）
            let write_rpc_info = format!("{:?}", self.write_client);
            if write_rpc_info.contains("localhost") || write_rpc_info.contains("127.0.0.1") {
                // サーバー環境: ローカルRPC使用
                "http://localhost:3001/evm".to_string()
            } else {
                // 開発環境: 外部RPC使用
                "http://5.104.84.211:3001/evm".to_string()
            }
        });
        
        info!("🔗 リアルタイムRPC設定: {}", realtime_rpc_url);
        
        // カスタムHTTPクライアントを作成（1秒ブロック用高速化設定）
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(2))  // 5秒 → 2秒に短縮（1秒ブロック対応）
            .tcp_keepalive(Some(Duration::from_secs(5)))  // 10秒 → 5秒に短縮
            .pool_max_idle_per_host(30)  // 20 → 30に増加（1秒ブロック用）
            .connect_timeout(Duration::from_millis(800))  // 2秒 → 800msに短縮
            .pool_idle_timeout(Some(Duration::from_secs(5)))  // 10秒 → 5秒に短縮
            .tcp_nodelay(true)
            .build()?;
        
        let url = reqwest::Url::parse(&realtime_rpc_url)?;
        let http = ethers::providers::Http::new_with_client(url, client);
        let mut provider = Provider::new(http);
        provider.set_interval(Duration::from_millis(50)); // 100ms → 50msに短縮（1秒ブロック対応）
        
        // 接続テスト
        match provider.get_block_number().await {
            Ok(block_num) => {
                info!("✅ リアルタイムRPC接続成功: 最新ブロック {} (URL: {})", block_num, realtime_rpc_url);
                self.realtime_client = Some(Arc::new(provider));
                Ok(())
            }
            Err(e) => {
                warn!("❌ リアルタイムRPC接続失敗 ({}): {}。アーカイブRPCにフォールバック", realtime_rpc_url, e);
                self.realtime_client = None;
                Ok(()) // 失敗してもエラーにしない（フォールバック）
            }
        }
    }

    // 清算実行前の厳格なバリデーション
    async fn validate_liquidation_before_execution<T: Middleware + 'static>(
        borrower: Address,
        collateral: Address,
        debt: Address,
        health_factor: U256,
        pool_data: &IPoolDataProvider<T>,
        write_client: Arc<T>,
        _config: &DeploymentConfig,  // 未使用変数にアンダースコアを追加
    ) -> Result<bool> {
        // 1. 健康状態の最終確認
        if health_factor >= U256::from(1000000000000000000u64) {
            info!("❌ 清算スキップ: ヘルスファクター {} >= 1.0", health_factor);
            return Ok(false);
        }

        // 2. 同一トークンの場合は必ずOK
        if collateral == debt {
            info!("✅ 同一トークン清算: バリデーションパス");
            return Ok(true);
        }

        // 3. 実績ペアの厳格チェック
        if !Self::is_validated_pair(collateral, debt) {
            info!("❌ 清算スキップ: 未検証ペア {} → {}", collateral, debt);
            return Ok(false);
        }

        // 4. 借り手の担保・債務残高チェック
        let (_, stable_debt, variable_debt, _, _, _, _, _, _) = pool_data
            .get_user_reserve_data(debt, borrower)
            .await
            .map_err(|e| {
                warn!("債務データ取得エラー: {}", e);
                e
            })?;

        let total_debt = stable_debt + variable_debt;
        if total_debt == U256::zero() {
            info!("❌ 清算スキップ: 借り手の債務残高が0");
            return Ok(false);
        }

        // 5. 担保残高チェック
        let collateral_config = Self::get_token_config(collateral);
        if let Some(config) = collateral_config {
            let a_token = IERC20::new(config.a_address, write_client);
            let collateral_balance = a_token.balance_of(borrower).await
                .map_err(|e| {
                    warn!("担保残高取得エラー: {}", e);
                    e
                })?;

            if collateral_balance == U256::zero() {
                info!("❌ 清算スキップ: 借り手の担保残高が0");
                return Ok(false);
            }
        }

        // 6. 最小清算額チェック
        let min_liquidation_amount = U256::from(1000000000000000u64); // 0.001 ETH相当
        if total_debt < min_liquidation_amount {
            info!("❌ 清算スキップ: 債務額が最小清算額 {} を下回る", min_liquidation_amount);
            return Ok(false);
        }

        info!("✅ 清算バリデーション成功: {} → {}", collateral, debt);
        Ok(true)
    }

    // 厳格に検証済みのペアのみを許可
    fn is_validated_pair(token_a: Address, token_b: Address) -> bool {
        let whype = Address::from_str("0x5555555555555555555555555555555555555555").unwrap();
        let usdt0 = Address::from_str("0xb8ce59fc3717ada4c02eadf9682a9e934f625ebb").unwrap();
        let ubtc = Address::from_str("0x9fdbda0a5e284c32744d2f17ee5c74b284993463").unwrap();
        let ueth = Address::from_str("0xbe6727b535545c67d5caa73dea54865b92cf7907").unwrap();

        // 超厳格: 100%確実なペアのみ許可
        let ultra_safe_pairs = vec![
            (whype, usdt0),   // WHYPE-USDT0 (最高流動性)
            (ubtc, usdt0),    // UBTC-USDT0 (確実)
            (ueth, usdt0),    // UETH-USDT0 (確実)
        ];

        ultra_safe_pairs.iter().any(|(a, b)| {
            (token_a == *a && token_b == *b) || (token_a == *b && token_b == *a)
        })
    }

    // トークン設定の取得
    fn get_token_config(token_address: Address) -> Option<TokenConfig> {
        // 主要トークンの設定を返す
        match token_address {
            addr if addr == Address::from_str("0x5555555555555555555555555555555555555555").unwrap() => {
                Some(TokenConfig {
                    address: addr,
                    a_address: addr, // WHYPEは自身がaToken
                    decimals: 18,
                    ltv: 8000,
                    liquidation_threshold: 8250,
                    liquidation_bonus: 10500,
                    reserve_factor: 1000,
                    protocol_fee: 1000,
                })
            },
            addr if addr == Address::from_str("0xb8ce59fc3717ada4c02eadf9682a9e934f625ebb").unwrap() => {
                Some(TokenConfig {
                    address: addr,
                    a_address: addr, // USDT0
                    decimals: 6,
                    ltv: 8000,
                    liquidation_threshold: 8500,
                    liquidation_bonus: 10400,
                    reserve_factor: 1000,
                    protocol_fee: 1000,
                })
            },
            addr if addr == Address::from_str("0x9fdbda0a5e284c32744d2f17ee5c74b284993463").unwrap() => {
                Some(TokenConfig {
                    address: addr,
                    a_address: addr, // UBTC
                    decimals: 8,
                    ltv: 7000,
                    liquidation_threshold: 7500,
                    liquidation_bonus: 11000,
                    reserve_factor: 2000,
                    protocol_fee: 1000,
                })
            },
            addr if addr == Address::from_str("0xbe6727b535545c67d5caa73dea54865b92cf7907").unwrap() => {
                Some(TokenConfig {
                    address: addr,
                    a_address: addr, // UETH
                    decimals: 18,
                    ltv: 8000,
                    liquidation_threshold: 8250,
                    liquidation_bonus: 10500,
                    reserve_factor: 1500,
                    protocol_fee: 1000,
                })
            },
            _ => None,
        }
    }

    // 初回スキャン専用アーカイブRPCクライアントの初期化
    pub async fn init_initial_scan_client(&mut self) -> Result<()> {
        info!("📚 初回スキャン専用アーカイブRPCクライアントの初期化を開始");
        
        // 初回スキャン専用アーカイブRPCのURL
        let archive_rpc_url = std::env::var("INITIAL_SCAN_ARCHIVE_RPC_URL")
            .unwrap_or_else(|_| "https://rpc.hyperlend.finance/archive".to_string());
        
        info!("🔗 初回スキャン用アーカイブRPC設定: {}", archive_rpc_url);
        
        // 初回スキャン用に最適化されたHTTPクライアントを作成
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(120))  // 120秒タイムアウト（大量データ取得用）
            .tcp_keepalive(Some(Duration::from_secs(30)))
            .pool_max_idle_per_host(10)  // 初回スキャンなので控えめ
            .connect_timeout(Duration::from_secs(10))
            .pool_idle_timeout(Some(Duration::from_secs(60)))
            .tcp_nodelay(true)
            .build()?;
        
        let url = reqwest::Url::parse(&archive_rpc_url)?;
        let http = ethers::providers::Http::new_with_client(url, client);
        let mut provider = Provider::new(http);
        provider.set_interval(Duration::from_millis(200)); // 200ms間隔（負荷軽減）
        
        // 接続テスト（詳細ログ付き）
        info!("🔍 初回スキャン用アーカイブRPCへの接続をテスト中...");
        match tokio::time::timeout(Duration::from_secs(15), provider.get_block_number()).await {
            Ok(Ok(block_num)) => {
                info!("✅ 初回スキャン用アーカイブRPC接続成功!");
                info!("   📊 RPC URL: {}", archive_rpc_url);
                info!("   🧱 最新ブロック番号: {}", block_num);
                info!("   ⚡ 専用RPCを使用した初回スキャンが有効になりました");
                self.initial_scan_client = Some(Arc::new(provider));
                Ok(())
            }
            Ok(Err(e)) => {
                warn!("❌ 初回スキャン用アーカイブRPC接続エラー:");
                warn!("   🌐 URL: {}", archive_rpc_url);
                warn!("   📄 エラー詳細: {}", e);
                warn!("   🔄 フォールバック: 通常のarchive_clientを使用します");
                self.initial_scan_client = None;
                Ok(()) // エラーでも継続（フォールバック利用）
            }
            Err(_) => {
                warn!("⏰ 初回スキャン用アーカイブRPC接続タイムアウト:");
                warn!("   🌐 URL: {}", archive_rpc_url);
                warn!("   ⏱️ タイムアウト時間: 15秒");
                warn!("   🔄 フォールバック: 通常のarchive_clientを使用します");
                self.initial_scan_client = None;
                Ok(()) // タイムアウトでも継続（フォールバック利用）
            }
        }
    }

    // 🆕 アーカイブRPCフォールバック専用供給ログ取得
    async fn get_supply_logs_with_archive_fallback(&self, block_number: u64) -> Result<Vec<SupplyFilter>> {
        if let Some(ref archive_client) = self.initial_scan_client {
            let pool = Pool::<Provider<ethers::providers::Http>>::new(self.config.pool_address, archive_client.clone());
            let filter = pool.supply_filter()
                .from_block(U64::from(block_number))
                .to_block(U64::from(block_number));
            
            match tokio::time::timeout(
                Duration::from_secs(15), // アーカイブRPCは長めのタイムアウト
                filter.query()
            ).await {
                Ok(result) => {
                    match result {
                        Ok(logs) => {
                            info!("✅ 専用アーカイブRPC供給ログ取得成功: {}件 (ブロック {})", logs.len(), block_number);
                            Ok(logs)
                        },
                        Err(e) => {
                            error!("専用アーカイブRPC供給ログ取得エラー: {}", e);
                            self.get_fallback_supply_logs(block_number).await
                        }
                    }
                },
                Err(_) => {
                    error!("専用アーカイブRPC供給ログ取得タイムアウト");
                    self.get_fallback_supply_logs(block_number).await
                }
            }
        } else {
            warn!("専用アーカイブRPCクライアントが利用できません。通常のフォールバックを使用");
            self.get_fallback_supply_logs(block_number).await
        }
    }

    // 🆕 アーカイブRPCフォールバック専用借入ログ取得
    async fn get_borrow_logs_with_archive_fallback(&self, block_number: u64) -> Result<Vec<BorrowFilter>> {
        if let Some(ref archive_client) = self.initial_scan_client {
            let pool = Pool::<Provider<ethers::providers::Http>>::new(self.config.pool_address, archive_client.clone());
            let filter = pool.borrow_filter()
                .from_block(U64::from(block_number))
                .to_block(U64::from(block_number));
            
            match tokio::time::timeout(
                Duration::from_secs(15), // アーカイブRPCは長めのタイムアウト
                filter.query()
            ).await {
                Ok(result) => {
                    match result {
                        Ok(logs) => {
                            info!("✅ 専用アーカイブRPC借入ログ取得成功: {}件 (ブロック {})", logs.len(), block_number);
                            Ok(logs)
                        },
                        Err(e) => {
                            error!("専用アーカイブRPC借入ログ取得エラー: {}", e);
                            self.get_fallback_borrow_logs(block_number).await
                        }
                    }
                },
                Err(_) => {
                    error!("専用アーカイブRPC借入ログ取得タイムアウト");
                    self.get_fallback_borrow_logs(block_number).await
                }
            }
        } else {
            warn!("専用アーカイブRPCクライアントが利用できません。通常のフォールバックを使用");
            self.get_fallback_borrow_logs(block_number).await
        }
    }

    // 初回スキャン専用：5000ブロックずつ効率的に借入ログを取得
    async fn get_initial_scan_borrow_logs(&self, from_block: u64, to_block: u64) -> Result<Vec<BorrowFilter>> {
        if let Some(ref archive_client) = self.initial_scan_client {
            let pool = Pool::<Provider<ethers::providers::Http>>::new(self.config.pool_address, archive_client.clone());
            let filter = pool.borrow_filter()
                .from_block(U64::from(from_block))
                .to_block(U64::from(to_block));
            
            match tokio::time::timeout(
                Duration::from_secs(60),
                filter.query()
            ).await {
                Ok(result) => result.map_err(|e| anyhow!("初回スキャン借入ログ取得エラー: {}", e)),
                Err(_) => Err(anyhow!("初回スキャン借入ログ取得タイムアウト")),
            }
        } else {
            // フォールバック: 通常のアーカイブクライアント使用
            let pool = Pool::<M>::new(self.config.pool_address, self.archive_client.clone());
            let filter = pool.borrow_filter()
                .from_block(U64::from(from_block))
                .to_block(U64::from(to_block));
            
            match tokio::time::timeout(
                Duration::from_secs(60),
                filter.query()
            ).await {
                Ok(result) => result.map_err(|e| anyhow!("借入ログ取得エラー: {}", e)),
                Err(_) => Err(anyhow!("借入ログ取得タイムアウト")),
            }
        }
    }

    // 初回スキャン専用：5000ブロックずつ効率的に供給ログを取得
    async fn get_initial_scan_supply_logs(&self, from_block: u64, to_block: u64) -> Result<Vec<SupplyFilter>> {
        if let Some(ref archive_client) = self.initial_scan_client {
            let pool = Pool::<Provider<ethers::providers::Http>>::new(self.config.pool_address, archive_client.clone());
            let filter = pool.supply_filter()
                .from_block(U64::from(from_block))
                .to_block(U64::from(to_block));
            
            match tokio::time::timeout(
                Duration::from_secs(60),
                filter.query()
            ).await {
                Ok(result) => result.map_err(|e| anyhow!("初回スキャン供給ログ取得エラー: {}", e)),
                Err(_) => Err(anyhow!("初回スキャン供給ログ取得タイムアウト")),
            }
        } else {
            // フォールバック: 通常のアーカイブクライアント使用
            let pool = Pool::<M>::new(self.config.pool_address, self.archive_client.clone());
            let filter = pool.supply_filter()
                .from_block(U64::from(from_block))
                .to_block(U64::from(to_block));
            
            match tokio::time::timeout(
                Duration::from_secs(60),
                filter.query()
            ).await {
                Ok(result) => result.map_err(|e| anyhow!("供給ログ取得エラー: {}", e)),
                Err(_) => Err(anyhow!("供給ログ取得タイムアウト")),
            }
        }
    }

    // 実際のプールコントラクトからヘルスファクターを取得
    async fn get_real_health_factor(&self, borrower: Address) -> Result<U256> {
        use bindings_aave::pool::Pool;
        
        let pool = Pool::new(self.config.pool_address, self.archive_client.clone());
        
        match pool.get_user_account_data(borrower).call().await {
            Ok(account_data) => {
                // getUserAccountDataは6つの値を返す: (totalCollateralBase, totalDebtBase, availableBorrowsBase, currentLiquidationThreshold, ltv, healthFactor)
                let health_factor = account_data.5; // 6番目の要素がhealth_factor
                
                // ヘルスファクターが0の場合（無限大を意味）、安全な大きな値を返す
                if health_factor.is_zero() {
                    // 債務がない場合は非常に大きな値（100.0に相当）を返す
                    Ok(U256::from_dec_str("100000000000000000000").unwrap()) // 100.0 with 18 decimals
                } else {
                    Ok(health_factor)
                }
            },
            Err(e) => {
                // コントラクト呼び出しエラーの場合
                Err(anyhow!("getUserAccountData呼び出しエラー: {}", e))
            }
        }
    }
}

// 実用的なスワップパス戦略（改良版）
fn create_practical_swap_path(collateral: Address, debt: Address, liq_path: &str) -> Result<Vec<u8>> {
    info!("🛡️ 超厳格モード: 検証済みペアのみ許可");
    
    // 同じトークンの場合は直接清算（スワップ不要）
    if collateral == debt {
        info!("✅ 直接清算: 同一トークン {} (スワップ不要)", collateral);
        return create_direct_liquidation_path(collateral);
    }
    
    // 🔒 超厳格: 100%確実なペアのみ使用
    if !AaveStrategy::<ethers::providers::Provider<ethers::providers::Http>>::is_validated_pair(collateral, debt) {
        warn!("❌ 未検証ペア検出: {} → {}。直接清算にフォールバック", collateral, debt);
        return create_direct_liquidation_path(collateral);
    }
    
    // 検証済みペアの直接スワップ
    info!("✅ 検証済みペア使用: {} → {}", collateral, debt);
    create_direct_swap_path(collateral, debt, liq_path)
}

// 既知のペアかどうかをチェック
fn is_known_pair(token_a: Address, token_b: Address, known_pairs: &[(Address, Address)]) -> bool {
    known_pairs.iter().any(|(a, b)| {
        (token_a == *a && token_b == *b) || (token_a == *b && token_b == *a)
    })
}

// 直接スワップパスを作成
fn create_direct_swap_path(token_in: Address, token_out: Address, liq_path: &str) -> Result<Vec<u8>> {
    match create_direct_swap_path_with_pool_type(token_in, token_out, liq_path, false) {
        Ok(path) => Ok(path),
        Err(e) => {
            // Kittenswapの場合、stable poolも試行
            if liq_path == "kittenswap" {
                warn!("Volatile poolで失敗: {}。Stable poolを試行", e);
                create_direct_swap_path_with_pool_type(token_in, token_out, liq_path, true)
            } else {
                Err(e)
            }
        }
    }
}

// プール種別を指定してスワップパスを作成
fn create_direct_swap_path_with_pool_type(token_in: Address, token_out: Address, liq_path: &str, is_stable: bool) -> Result<Vec<u8>> {
    let mut path = Vec::new();
    
    match liq_path {
        "kittenswap" => {
            path.extend_from_slice(&token_in.0);
            if is_stable {
                path.push(1u8);  // stable pool（stable=true）
                info!("Kittenswap stableパス: {} → {} ({}バイト)", token_in, token_out, path.len() + 21);
            } else {
                path.push(0u8);  // volatile pool（stable=false）
                info!("Kittenswap volatileパス: {} → {} ({}バイト)", token_in, token_out, path.len() + 21);
            }
            path.extend_from_slice(&token_out.0);
        },
        "hyperswap" => {
            path.extend_from_slice(&token_in.0);
            let fee_bytes = 3000u32.to_be_bytes();
            path.extend_from_slice(&fee_bytes[1..4]);
            path.extend_from_slice(&token_out.0);
            info!("Hyperswap直接パス: {} → {} ({}バイト)", token_in, token_out, path.len());
        },
        _ => {
            // デフォルトはKittenswap volatile
            path.extend_from_slice(&token_in.0);
            path.push(0u8);
            path.extend_from_slice(&token_out.0);
        }
    }
    
    Ok(path)
}

// マルチホップパスを作成（中継トークン経由）
fn create_multi_hop_path(token_in: Address, intermediate: Address, token_out: Address, liq_path: &str) -> Result<Vec<u8>> {
    let mut path = Vec::new();
    
    match liq_path {
        "kittenswap" => {
            // token_in → intermediate
            path.extend_from_slice(&token_in.0);
            path.push(0u8);  // volatile pool
            path.extend_from_slice(&intermediate.0);
            // intermediate → token_out  
            path.push(0u8);  // volatile pool
            path.extend_from_slice(&token_out.0);
            info!("Kittenswapマルチホップ: {} → {} → {} ({}バイト)", token_in, intermediate, token_out, path.len());
        },
        "hyperswap" => {
            // token_in → intermediate
            path.extend_from_slice(&token_in.0);
            let fee_bytes = 3000u32.to_be_bytes();
            path.extend_from_slice(&fee_bytes[1..4]);
            path.extend_from_slice(&intermediate.0);
            // intermediate → token_out
            path.extend_from_slice(&fee_bytes[1..4]);
            path.extend_from_slice(&token_out.0);
            info!("Hyperswapマルチホップ: {} → {} → {} ({}バイト)", token_in, intermediate, token_out, path.len());
        },
        _ => {
            // デフォルト: Kittenswap
            path.extend_from_slice(&token_in.0);
            path.push(0u8);
            path.extend_from_slice(&intermediate.0);
            path.push(0u8);
            path.extend_from_slice(&token_out.0);
        }
    }
    
    Ok(path)
}

// 直接清算パス（スワップなし）
fn create_direct_liquidation_path(token: Address) -> Result<Vec<u8>> {
    // 最小限のパス: トークンアドレスのみ
    let mut path = Vec::new();
    path.extend_from_slice(&token.0);
    
    info!("直接清算パス: {} ({}バイト)", token, path.len());
    Ok(path)
}

// IOAエラー対策付きの最終フォールバック
fn create_swap_path_with_ioa_protection(collateral: Address, debt: Address, liq_path: &str) -> Result<Vec<u8>> {
    // まず実用的なパスを試行
    match create_practical_swap_path(collateral, debt, liq_path) {
        Ok(path) => {
            info!("実用的パス作成成功: {}バイト", path.len());
            return Ok(path);
        },
        Err(e) => {
            warn!("実用的パス作成失敗: {}。IOA対策フォールバックを試行", e);
        }
    }
    
    // IOA対策: 異なるDEXを試行
    let alternative_liq_path = match liq_path {
        "kittenswap" => "hyperswap",
        "hyperswap" => "kittenswap",
        _ => "kittenswap"
    };
    
    match create_practical_swap_path(collateral, debt, alternative_liq_path) {
        Ok(path) => {
            warn!("代替DEXパス成功: {} ({}バイト)", alternative_liq_path, path.len());
            return Ok(path);
        },
        Err(e) => {
            warn!("代替DEXパス失敗: {}", e);
        }
    }
    
    // 最終手段: 直接清算のみ（スワップ無効化）
    warn!("全てのスワップ戦略が失敗。直接清算のみを使用");
    create_direct_liquidation_path(collateral)
}

// 本番/開発環境の判定
#[derive(Debug, Clone)]
pub enum EnvironmentMode {
    Development,
    Production,
}

impl EnvironmentMode {
    pub fn from_rpc_url(rpc_url: &str) -> Self {
        if rpc_url.contains("localhost") || rpc_url.contains("127.0.0.1") {
            Self::Production  // サーバー内のローカルRPC = 本番モード
        } else {
            Self::Development // リモートRPC = 開発モード
        }
    }
    
    pub fn get_optimized_settings(&self) -> OptimizedSettings {
        match self {
            Self::Development => OptimizedSettings {
                timeout: MAIN_RPC_TIMEOUT,
                retry_delay: RETRY_DELAY,
                multicall_chunk_size: MULTICALL_CHUNK_SIZE,
                max_parallel_tasks: MAX_PARALLEL_TASKS,
                log_level: "INFO",
                buffer_multiplier: 1,
            },
            Self::Production => OptimizedSettings {
                timeout: SERVER_MODE_TIMEOUT,
                retry_delay: SERVER_MODE_RETRY_DELAY,
                multicall_chunk_size: SERVER_MULTICALL_CHUNK_SIZE,
                max_parallel_tasks: SERVER_MAX_PARALLEL_TASKS,
                log_level: "WARN",  // 本番は警告以上のみ
                buffer_multiplier: 2,  // バッファサイズ2倍
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct OptimizedSettings {
    pub timeout: u64,
    pub retry_delay: u64,
    pub multicall_chunk_size: usize,
    pub max_parallel_tasks: usize,
    pub log_level: &'static str,
    pub buffer_multiplier: usize,
}
