# Aave V3 Liquidator

This is an [Artemis](https://github.com/paradigmxyz/artemis) bot that liquidates aave v3 positions

Based on work by marktoda (https://github.com/marktoda/artemis-aave-v3-liquidator),  Seamless Protocol (https://github.com/seamless-protocol/liquidator) and Hypurrfi team (https://github.com/lastdotnet/hypurrfi-liquidator)

# Getting Started

1. Install Rust [ref](https://doc.rust-lang.org/book/ch01-01-installation.html)
    - Install deps: `sudo apt install -y build-essential libssl-dev` (Linux) or `xcode-select --install` (Mac)

3. Install Foundry [ref](https://book.getfoundry.sh/getting-started/installation)

3. Deploy liquidator contract
```bash
cd crates/liquidator-contract
forge install
forge create ./src/Liquidator.sol:Liquidator --private-key <xyz> --rpc-url <xyz> --broadcast
```

for mainnet verification:
```bash
forge create ./src/Liquidator.sol:Liquidator --private-key <xyz> --rpc-url <xyz> --broadcast --verify --verifier sourcify --verifier-url https://sourcify.parsec.finance/verify
```

4. Build Rust Application

a. cd to root of project

b. (if not cross compiling) Compile binary for host arch: 
```bash
cargo build --release
```

c. (if cross compiling) Compile binary on Mac for Linux: 
Cross is a tool developed by Rust devs to simplify cross compilation (compiling on one machine architecture for another architecture, e.g.: compile binary on mac for running on a linux machine). Make sure to install from github repo not latest release in cargo (it's quite outdated). [Github link](https://github.com/cross-rs/cross)

If you are using an ARM based Mac (e.g.: M1), you need to set DOCKER_DEFAULT_PLATFORM env to `linux/amd64`

```bash
cargo install cross --git https://github.com/cross-rs/cross
DOCKER_DEFAULT_PLATFORM=linux/amd64 cross build --target x86_64-unknown-linux-gnu --release
```

d. Copy (scp) binary to liquidator host

6. Run liquidator
```bash
./target/release/aave-v3-liquidator \
    --archive-rpc <xyz> \
    --write-rpc <xyz> \
    --private-key <xyz> \
    --bid-percentage 100 \
    --deployment hyperlend \
    --liquidator-address <xyz> \
    --chain-id <xyz>
```

- Write rpc: https://rpc.hyperliquid.xyz/evm
- Archive rpc: https://rpc.hyperlend.finance/archive


# Re-generate Contract Bindings

## Liquidator
```bash
forge bind --bindings-path ./crates/bindings-liquidator --root ./crates/liquidator-contract --crate-name bindings-liquidator --overwrite
```

## Aave
```bash
forge bind --bindings-path ./crates/bindings-aave --root {path to aave-v3-core} --crate-name bindings-aave --overwrite
```

## 清算ボット改善点（2023年最新版）

### 主な改善機能

#### 1. 階層的並列スキャン
- ヘルスファクターに基づいた5つの階層（Critical, High Risk, Medium Risk, Low Risk, Safe）
- 各階層ごとに最適なスキャン間隔と並列度を設定
- 効率的な二段階並列処理（階層間の並列処理 + 階層内の並列処理）
- 危険な状態の借り手を優先的にスキャン

#### 2. 動的スキャン間隔調整
- 危険ゾーンの借り手が検出された場合は自動的にスキャン間隔を短縮（5秒）
- 安全な状態では基本間隔（30秒）を維持
- タイムコレクターによるイベント生成間隔の動的調整

#### 3. 並列トランザクション処理
- 最大5つの清算トランザクションを同時に処理
- スレッドセーフなノンス管理による並列トランザクション送信
- 同一借り手の重複防止メカニズム
- 失敗したトランザクションの検出と管理

#### 4. 最適化されたパフォーマンス
- Multicallによる効率的なデータ取得
- 階層ごとのリソース管理（並列度の最適化）
- 実行時間の計測と報告
- 適切なエラーハンドリングとロギング

#### 5. 利益最適化と安全性
- 最小利益閾値（0.5 USDT）の設定
- 最大ガス価格（50 Gwei）の制限
- BidPercentageを50%に設定
- トランザクションタイムアウト処理（2分）
- ノンスの定期的な再同期

### パフォーマンス
AMD Ryzen 7 5700G（8コア）、16GB RAMの環境で、1,000人以上の借り手のモニタリングを効率的に実行できることを確認済み。

### 使用例
```bash
./start-liquidator.sh --archive-rpc "https://your-archive-node" --write-rpc "https://your-write-node" --private-key "your-private-key" --bid-percentage 50 --deployment HYPERLEND --liquidator-address "your-liquidator-contract-address" --chain-id 1101
```

詳細な設定は`ExecutionConfig`および`ScanState`構造体内で調整できます。必要に応じてパラメータをカスタマイズしてください。

## パフォーマンスチューニング

### チャネルバッファサイズの調整

Artemisエンジンのチャネルラグ問題に対処するため、このプロジェクトではチャネルバッファサイズを調整できるようになっています。デフォルトでは、イベントチャネルとアクションチャネルのバッファサイズは10000に設定されていますが、これはコマンドラインオプションで変更できます：

```bash
cargo run -- --archive-rpc <RPC_URL> --write-rpc <RPC_URL> --private-key <KEY> --bid-percentage <VALUE> --deployment <DEPLOYMENT> --liquidator-address <ADDRESS> --chain-id <CHAIN_ID> --event-buffer-size 20000 --action-buffer-size 20000
```

バッファサイズを増やすと、イベント処理の遅延が軽減されますが、メモリ使用量が増加することに注意してください。最適な値はワークロードと利用可能なシステムリソースによって異なります。

### チャネルラグのモニタリング

清算ボットの診断ツールを使用して、チャネルラグをモニタリングできます：

```bash
cargo run --bin diagnose -- --log-file <LOG_FILE_PATH> --output-file diagnosis_report.txt
```

このモニタリングツールは、チャネルラグの頻度と重大度を追跡し、バッファサイズの調整が効果的かどうかを評価するのに役立ちます。

# Hyperliquid 清算ボット

Hyperliquid DEXの自動清算ボット。

## チャネルラグ問題の診断と対策

このプロジェクトでは、Artemisフレームワーク上で動作する清算ボットのチャネルラグ問題に対する診断と解決策を実装しています。

### 診断ツール

#### チャネルラグ分析ツール

```bash
cargo run --bin liquidator-diagnose analyze --log-file path/to/log.log --duration 3600 --report-interval 300
```

オプション:
- `--log-file`: ログファイルのパス（必須）
- `--duration`: 診断の実行時間（秒）（デフォルト: 3600）
- `--report-interval`: レポート間隔（秒）（デフォルト: 300）
- `--output-file`: 診断結果の出力ファイル（オプション）

#### チャネルバッファテストツール

```bash
cargo run --bin channel-test --min-buffer 10000 --max-buffer 5000000 --producers 3 --consumers 1
```

オプション:
- `--min-buffer`: 最小バッファサイズ（デフォルト: 10000）
- `--max-buffer`: 最大バッファサイズ（デフォルト: 5000000）
- `--buffer-multiplier`: バッファサイズの増加係数（デフォルト: 5）
- `--min-rate`: 最小生成者スループット（メッセージ/秒）（デフォルト: 100）
- `--max-rate`: 最大生成者スループット（メッセージ/秒）（デフォルト: 2000）
- `--consumer-delay`: 消費者の平均処理時間（ミリ秒）（デフォルト: 5）
- `--producers`: 生成者並列数（デフォルト: 3）
- `--consumers`: 消費者並列数（デフォルト: 1）
- `--duration`: テスト時間（秒）（デフォルト: 30）
- `--output`: 出力ファイル（オプション）

### 対策の概要

1. **チャネルバッファサイズの拡大**
   - 10,000から5,000,000へ増加
   - スパイク時のイベント損失を防止

2. **階層別並列処理の最適化**
   - Critical階層: 並列度4→3
   - High Risk階層: 並列度3→2
   - Medium Risk階層: 並列度2→1
   - Low Risk階層: 並列度1（変更なし）

3. **優先度キューの実装**
   - Redis Sorted Setによる優先度管理
   - ヘルスファクターが低い順に処理

4. **キャッシュ層の導入**
   - 動的TTL設定によるヘルスファクターのキャッシュ
   - 重複スキャンの削減

### パフォーマンス改善

- 診断結果から、チャネルラグの92.9%が100,000以上の値であったことがわかりました
- バッファサイズ拡大と並列度調整により、チャネルラグが大幅に減少
- 優先度キューの導入で、水没リスクの高い借り手を優先的に処理
- キャッシュ層により、重複スキャンを防止し全体的なスループットを向上

## 開発者向け情報

### デバッグ方法

```bash
# デバッグビルド
cargo build

# チャネル診断テスト
cargo run --bin channel-test -- --duration 10 --min-buffer 10000 --max-buffer 1000000

# ログ解析
cargo run --bin liquidator-diagnose analyze --log-file ./liquidator.log --duration 600
```
