// use std::sync::Arc;
use std::collections::HashMap;
use anyhow::{Result, anyhow};
use ethers::types::{Address, U256};
use redis::{aio::ConnectionManager, AsyncCommands};
use tracing::{info, debug, error, warn};
use std::fmt;

/// 優先度順に整列された借り手のキュー
pub struct PriorityQueue {
    /// Redisコネクション
    conn: ConnectionManager,
    /// Sorted Setのキー
    queue_key: String,
}

// ConnectionManagerにはDebugトレイトが実装されていないため、手動でDebugを実装
impl fmt::Debug for PriorityQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PriorityQueue")
            .field("queue_key", &self.queue_key)
            .field("conn", &"<ConnectionManager>")
            .finish()
    }
}

impl PriorityQueue {
    /// 新しい優先度キューを作成
    pub async fn new(redis_url: &str, queue_key: &str) -> Result<Self> {
        let client = redis::Client::open(redis_url)?;
        let conn = ConnectionManager::new(client).await?;
        
        Ok(Self {
            conn,
            queue_key: queue_key.to_string(),
        })
    }
    
    /// 借り手をキューに追加（スコアはヘルスファクター）
    /// ヘルスファクターが低いほど優先度が高い
    pub async fn add_borrower(&mut self, borrower: Address, health_factor: U256) -> Result<()> {
        // U256をf64に変換（1.0 = 10^18）
        let score = health_factor.as_u128() as f64 / 1e18;
        
        // アドレスを完全な形式で保存
        let addr_str = format!("0x{:x}", borrower);
        
        // ZADD queue_key score member
        let _: () = self.conn.zadd(&self.queue_key, addr_str, score).await?;
        
        debug!("借り手 {:?} をキューに追加しました（HF={}, スコア={}）", borrower, health_factor, score);
        Ok(())
    }
    
    /// 借り手のバッチをキューに追加
    pub async fn add_borrowers_batch(&mut self, borrowers: Vec<(Address, U256)>) -> Result<()> {
        if borrowers.is_empty() {
            return Ok(());
        }
        
        // パイプラインを作成
        let mut pipe = redis::pipe();
        
        // 処理するアイテム数を保存
        let batch_size = borrowers.len();
        
        // 各借り手をZADDコマンドでキューに追加
        for (borrower, health_factor) in borrowers {
            // U256をf64に変換
            let score = health_factor.as_u128() as f64 / 1e18;
            
            // アドレスを完全な形式で保存
            let addr_str = format!("0x{:x}", borrower);
            
            // ZADDをパイプラインに追加
            pipe.zadd(&self.queue_key, addr_str, score);
        }
        
        // パイプラインを一度に実行
        let _: () = pipe.query_async(&mut self.conn).await?;
        
        info!("{}件の借り手をキューに追加/更新しました", batch_size);
        Ok(())
    }
    
    /// 借り手をキューから削除
    pub async fn remove_borrower(&mut self, borrower: Address) -> Result<()> {
        // ZREM queue_key member
        let _: () = self.conn.zrem(&self.queue_key, borrower.to_string()).await?;
        
        debug!("借り手 {:?} をキューから削除しました", borrower);
        Ok(())
    }
    
    /// 複数の借り手をキューから削除
    pub async fn remove_borrowers_batch(&mut self, borrowers: &[Address]) -> Result<()> {
        if borrowers.is_empty() {
            return Ok(());
        }
        
        // 各借り手の文字列表現
        let members: Vec<String> = borrowers.iter().map(|addr| addr.to_string()).collect();
        
        // ZREM queue_key member1 member2 ...
        let removed: usize = self.conn.zrem(&self.queue_key, members).await?;
        
        debug!("{}件の借り手をキューから削除しました", removed);
        Ok(())
    }
    
    /// 優先度の高い（ヘルスファクターの低い）借り手を取得
    pub async fn get_highest_priority(&mut self, count: isize) -> Result<Vec<(Address, f64)>> {
        // ZRANGE queue_key 0 count-1 WITHSCORES
        let results: Vec<(String, f64)> = redis::cmd("ZRANGE")
            .arg(&self.queue_key)
            .arg(0)
            .arg(count - 1)
            .arg("WITHSCORES")
            .query_async(&mut self.conn)
            .await?;
        
        // 結果を変換
        let mut borrowers = Vec::with_capacity(results.len());
        for (addr_str, score) in results {
            match addr_str.parse::<Address>() {
                Ok(addr) => borrowers.push((addr, score)),
                Err(e) => {
                    warn!("無効なアドレス形式: {} - {}", addr_str, e);
                }
            }
        }
        
        Ok(borrowers)
    }
    
    /// キューのサイズを取得
    pub async fn size(&mut self) -> Result<usize> {
        // ZCARD queue_key
        let size: usize = self.conn.zcard(&self.queue_key).await?;
        Ok(size)
    }
    
    /// 指定されたスコア範囲内の借り手数を取得
    pub async fn count_in_range(&mut self, min: f64, max: f64) -> Result<usize> {
        // ZCOUNT queue_key min max
        let count: usize = self.conn.zcount(&self.queue_key, min, max).await?;
        Ok(count)
    }
    
    /// 借り手のスコア（ヘルスファクター）を取得
    pub async fn get_borrower_priority(&mut self, borrower: Address) -> Result<Option<f64>> {
        // ZSCORE queue_key member
        let score: Option<f64> = self.conn.zscore(&self.queue_key, borrower.to_string()).await?;
        Ok(score)
    }
    
    /// 指定されたヘルスファクター範囲内の借り手を取得
    pub async fn get_borrowers_in_range(&mut self, min: f64, max: f64) -> Result<Vec<(Address, f64)>> {
        // ZRANGEBYSCORE queue_key min max WITHSCORES
        let results: Vec<(String, f64)> = redis::cmd("ZRANGEBYSCORE")
            .arg(&self.queue_key)
            .arg(min)
            .arg(max)
            .arg("WITHSCORES")
            .query_async(&mut self.conn)
            .await?;
        
        // 結果を変換
        let mut borrowers = Vec::with_capacity(results.len());
        for (addr_str, score) in results {
            match addr_str.parse::<Address>() {
                Ok(addr) => borrowers.push((addr, score)),
                Err(e) => {
                    warn!("無効なアドレス形式: {} - {}", addr_str, e);
                }
            }
        }
        
        Ok(borrowers)
    }
} 