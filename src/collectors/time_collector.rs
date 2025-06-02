use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use futures::StreamExt;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;
use tokio::time::Duration;
use tokio_stream::wrappers::ReceiverStream;

/// A collector that generates a stream of tick events at configurable intervals.
#[derive(Debug)]
pub struct TimeCollector {
    pub poll_secs: u64,
    interval: Arc<Mutex<u64>>,
    interval_tx: Arc<Mutex<Option<mpsc::Sender<u64>>>>,
}

/// A new tick event, containing the timestamp.
#[derive(Debug, Clone)]
pub struct NewTick {
    pub timestamp: u64,
}

impl TimeCollector {
    pub fn new(poll_secs: u64) -> Self {
        Self { 
            poll_secs,
            interval: Arc::new(Mutex::new(poll_secs)),
            interval_tx: Arc::new(Mutex::new(None)),
        }
    }

    /// 動的にスキャン間隔を調整するためのメソッド
    pub fn adjust_interval(&self, new_interval_secs: u64) -> Result<()> {
        // 現在の間隔を更新
        {
            let mut interval = self.interval.lock().unwrap();
            *interval = new_interval_secs;
        }

        // 送信チャネルが設定されていれば、新しい間隔を送信
        let tx_guard = self.interval_tx.lock().unwrap();
        if let Some(tx) = &*tx_guard {
            match tx.try_send(new_interval_secs) {
                Ok(_) => Ok(()),
                Err(e) => {
                    // エラータイプを確認
                    match e {
                        tokio::sync::mpsc::error::TrySendError::Full(_) => {
                            // チャネルがいっぱいの場合は無視して続行
                            Ok(())
                        }
                        tokio::sync::mpsc::error::TrySendError::Closed(_) => {
                            // チャネルが閉じられている場合はエラー
                            Err(anyhow::anyhow!("Channel is closed"))
                        }
                    }
                }
            }
        } else {
            Ok(())
        }
    }

    /// 現在のスキャン間隔を取得
    pub fn get_current_interval(&self) -> u64 {
        *self.interval.lock().unwrap()
    }
}

/// Implementation of the [Collector](Collector) trait for the [TimeCollector](TimeCollector).
#[async_trait]
impl Collector<NewTick> for TimeCollector {
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, NewTick>> {
        // 間隔調整用のチャネルを作成
        let (tx, rx) = mpsc::channel::<u64>(10);
        
        // このインスタンスに送信チャネルを保存
        {
            let mut interval_tx = self.interval_tx.lock().unwrap();
            *interval_tx = Some(tx);
        }

        // 初期間隔でインターバルを作成
        let interval = tokio::time::interval(Duration::from_secs(self.poll_secs));
        
        // 動的に調整可能なイベントストリームを作成
        let rx_stream = ReceiverStream::new(rx);
        let interval_handle = self.interval.clone();
        
        let stream = futures::stream::unfold(
            (interval, rx_stream),
            move |(mut interval, mut rx_stream)| {
                let interval_handle = interval_handle.clone();
                async move {
                    tokio::select! {
                        // インターバルの時間が来た場合
                        _ = interval.tick() => {
                            let timestamp: u64 = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .expect("Invalid timestamp")
                                .as_secs();
                            let event = NewTick { timestamp };
                            Some((event, (interval, rx_stream)))
                        }
                        // 新しい間隔が設定された場合
                        Some(new_interval) = rx_stream.next() => {
                            // インターバルをリセット
                            interval = tokio::time::interval(Duration::from_secs(new_interval));
                            let timestamp: u64 = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .expect("Invalid timestamp")
                                .as_secs();
                            let event = NewTick { timestamp };
                            Some((event, (interval, rx_stream)))
                        }
                        // それ以外の場合（チャネルが閉じられた場合など）
                        _ = futures::future::ready(()) => {
                            let current_interval = *interval_handle.lock().unwrap();
                            interval = tokio::time::interval(Duration::from_secs(current_interval));
                            let timestamp: u64 = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .expect("Invalid timestamp")
                                .as_secs();
                            let event = NewTick { timestamp };
                            Some((event, (interval, rx_stream)))
                        }
                    }
                }
            },
        );

        Ok(Box::pin(stream))
    }
}
