# Hyperliquid清算ボット（Liquidator）設定ガイド

## 前提条件

- Rust 1.70以上
- Python 3.8以上（診断ツール用）
- Redis 6.0以上
- Cargo
- Git

## インストール

### 1. リポジトリのクローン

```bash
git clone https://github.com/your-org/hyperliquid-liquidator.git
cd hyperliquid-liquidator
```

### 2. 依存関係のインストール

```bash
# Rustの依存関係
cargo build --release

# Pythonの依存関係（診断ツール用）
pip install pandas matplotlib seaborn numpy
```

### 3. Redisのセットアップ

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install redis-server

# 設定ファイルを編集
sudo nano /etc/redis/redis.conf

# 起動
sudo systemctl start redis-server
```

Redisの設定(`redis.conf`)で以下のパラメータを調整してください：

```
maxmemory 2gb
maxmemory-policy allkeys-lru
```

## 設定

### 環境変数の設定

`.env`ファイルをプロジェクトルートに作成し、以下の内容を追加します：

```env
# 接続情報
ARCHIVE_RPC=https://your-archive-node-url
WRITE_RPC=https://your-write-node-url
PRIVATE_KEY=your-private-key

# 清算設定
BID_PERCENTAGE=80
CHAIN_ID=1337
DEPLOYMENT=HYPERLEND
LIQUIDATOR_ADDRESS=0x...

# Redis設定
REDIS_URL=redis://localhost:6379
CACHE_PREFIX=hyperliquid:liquidator

# パフォーマンス設定
POLL_INTERVAL_SECS=15
EVENT_BUFFER_SIZE=5000000
ACTION_BUFFER_SIZE=5000000
```

### 設定ファイルのカスタマイズ

`config/default.json`ファイルを作成して以下の内容を設定します：

```json
{
  "redis": {
    "url": "redis://localhost:6379",
    "cache_prefix": "hyperliquid:liquidator"
  },
  "performance": {
    "cache_ttl_settings": {
      "rapid": 30,
      "moderate": 120,
      "slow": 300,
      "stable": 600
    },
    "scanner": {
      "critical_parallelism": 4,
      "normal_parallelism": 2,
      "max_concurrent_batches": 5,
      "max_batch_queue_size": 200
    },
    "priority_queue": {
      "min_scan_interval": 5,
      "max_scan_interval": 900,
      "event_scan_health_threshold": 1.1
    }
  },
  "maintenance": {
    "interval": 300
  }
}
```

## 実行

### 清算ボットの起動

```bash
cargo run --release -- \
  --archive-rpc $ARCHIVE_RPC \
  --write-rpc $WRITE_RPC \
  --private-key $PRIVATE_KEY \
  --bid-percentage $BID_PERCENTAGE \
  --chain-id $CHAIN_ID \
  --deployment $DEPLOYMENT \
  --liquidator-address $LIQUIDATOR_ADDRESS \
  --poll-interval-secs $POLL_INTERVAL_SECS \
  --event-buffer-size $EVENT_BUFFER_SIZE \
  --action-buffer-size $ACTION_BUFFER_SIZE
```

または、環境変数を使用する場合：

```bash
source .env
cargo run --release
```

### 診断ツールの実行

チャネルラグの問題を診断する場合は、以下のコマンドを実行します：

```bash
# ログファイルからチャネルラグを診断
python channel_lag_diagnosis.py
```

診断結果は`diagnosis_output`ディレクトリに保存され、グラフと推奨事項が含まれます。

## チューニングとトラブルシューティング

### パフォーマンス調整

チャネルラグが発生する場合は、以下のパラメータを調整してください：

1. **イベントバッファサイズ**：大量のイベントがある場合は`EVENT_BUFFER_SIZE`を増やす（例：10000000）
2. **アクションバッファサイズ**：多くのアクションがあるバースト時には`ACTION_BUFFER_SIZE`を増やす
3. **並列度**：CPUコア数に応じて`critical_parallelism`と`normal_parallelism`を調整
4. **ポーリング間隔**：必要に応じて`POLL_INTERVAL_SECS`を調整（短くしすぎないよう注意）

### Redisのチューニング

Redisのパフォーマンスが重要です。以下の設定を検討してください：

```
# redis.conf
maxmemory-samples 10
activedefrag yes
hz 100
```

### メモリ使用量の最適化

メモリ使用量が多い場合は：

1. キャッシュTTLを短くする
2. 安定した借り手のスキャン頻度を下げる
3. バッチサイズを小さくする

### ログ解析

問題が発生した場合、ログをチェックして問題を特定してください：

```bash
grep "ERROR" liquidator.log > errors.log
grep "channel lagged" liquidator.log > channel_lag.log
```

## 統合アプローチの使用

3つの最適化アプローチ（イベント駆動型優先度キュー、マルチレベルスキャン、インクリメンタルキャッシュ）をすべて有効にするには、`AaveStrategy`の初期化コードが統合戦略を使用していることを確認してください。

コードでは以下のように自動的に統合戦略が初期化されます：

```rust
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
```

## ヘルスモニタリング

ボットが正常に動作しているか確認するために、以下のヘルスチェックを定期的に行ってください：

1. Redisメモリ使用量: `redis-cli info memory | grep used_memory_human`
2. チャネルラグの発生: ログで "channel lagged" を検索
3. 清算成功率: 成功した清算トランザクションの数をモニター

## 更新とアップグレード

最新の改善を取り込むには、リポジトリを定期的に更新してください：

```bash
git pull
cargo build --release
```

新しいインテグレーションや変更点がある場合は、`CHANGELOG.md`を参照してください。 