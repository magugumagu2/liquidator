use std::sync::Arc;
use std::time::{Duration, SystemTime};
use anyhow::{Result, anyhow};
use ethers::types::{Address, U256};
use redis::{aio::ConnectionManager, AsyncCommands};
use tracing::{info, error, warn, debug};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::fmt;

/// ヘルスファクターとスキャン結果のキャッシュ
pub struct BorrowerCache {
    /// Redisコネクション
    conn: ConnectionManager,
    /// ヘルスファクターのキャッシュキープレフィックス
    hf_prefix: String,
    /// 最終スキャン時刻のキャッシュキープレフィックス
    last_scan_prefix: String,
    /// 変化率のキャッシュキープレフィックス
    change_rate_prefix: String,
    /// 優先度キューのキー名
    priority_queue_key: String,
}

// ConnectionManagerにはDebugトレイトが実装されていないため、手動でDebugを実装
impl fmt::Debug for BorrowerCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BorrowerCache")
            .field("hf_prefix", &self.hf_prefix)
            .field("last_scan_prefix", &self.last_scan_prefix)
            .field("change_rate_prefix", &self.change_rate_prefix)
            .field("priority_queue_key", &self.priority_queue_key)
            .field("conn", &"<ConnectionManager>")
            .finish()
    }
}

impl BorrowerCache {
    /// 新しいキャッシュを作成
    pub async fn new(redis_url: &str, prefix: &str) -> Result<Self> {
        let client = redis::Client::open(redis_url)?;
        let conn = ConnectionManager::new(client).await?;
        
        Ok(Self {
            conn,
            hf_prefix: format!("{}:hf:", prefix),
            last_scan_prefix: format!("{}:last_scan:", prefix),
            change_rate_prefix: format!("{}:cr:", prefix),
            priority_queue_key: format!("{}:priority_queue", prefix),
        })
    }
    
    /// キャッシュの接続テスト
    pub async fn init(&mut self) -> Result<()> {
        // 接続テスト
        let ping: String = redis::cmd("PING")
            .query_async(&mut self.conn)
            .await?;
        
        if ping != "PONG" {
            return Err(anyhow!("Redisサーバーに接続できません"));
        }
        
        info!("借り手キャッシュを初期化しました");
        Ok(())
    }
    
    /// 借り手のヘルスファクターをキャッシュに保存
    /// TTLはヘルスファクターに基づいて動的に設定
    pub async fn cache_health_factor(&mut self, borrower: Address, health_factor: U256) -> Result<()> {
        let key = format!("{}{:?}", self.hf_prefix, borrower);
        let value = health_factor.to_string();
        
        // ヘルスファクターに基づいてTTLを設定
        let ttl = self.get_ttl_for_health_factor(&health_factor);
        
        // パイプラインを作成してアトミックに更新
        let mut pipe = redis::pipe();
        
        // ヘルスファクターをキャッシュに保存
        pipe.set_ex(&key, &value, ttl.as_secs() as usize);
        
        // 優先度キューに追加（スコアはヘルスファクターの浮動小数点表現）
        // スコアが低いほど優先度が高い（小さいヘルスファクターが先に取得される）
        let score = health_factor.as_u128() as f64 / 1e18;
        pipe.zadd(&self.priority_queue_key, borrower.to_string(), score);
        
        // パイプラインを実行
        let _: () = pipe.query_async(&mut self.conn).await?;
        
        debug!("借り手 {:?} のHFをキャッシュに保存: HF={}, TTL={:?}", borrower, value, ttl);
        
        Ok(())
    }
    
    /// 複数の借り手のヘルスファクターをバッチでキャッシュに保存
    pub async fn cache_health_factors_batch(&mut self, borrowers: &[(Address, U256)]) -> Result<()> {
        if borrowers.is_empty() {
            return Ok(());
        }
        
        // パイプラインを作成
        let mut pipe = redis::pipe();
        
        for (borrower, health_factor) in borrowers {
            let key = format!("{}{:?}", self.hf_prefix, borrower);
            let value = health_factor.to_string();
            let ttl = self.get_ttl_for_health_factor(health_factor).as_secs() as usize;
            
            // ヘルスファクターをキャッシュに保存
            pipe.set_ex(&key, value, ttl);
            
            // 優先度キューに追加
            let score = health_factor.as_u128() as f64 / 1e18;
            pipe.zadd(&self.priority_queue_key, borrower.to_string(), score);
        }
        
        // パイプラインを実行
        let _: () = pipe.query_async(&mut self.conn).await?;
        
        info!("{}件の借り手のHFをキャッシュに保存しました", borrowers.len());
        Ok(())
    }
    
    /// キャッシュから借り手のヘルスファクターを取得
    pub async fn get_health_factor(&mut self, borrower: &Address) -> Result<Option<U256>> {
        let key = format!("{}{:?}", self.hf_prefix, borrower);
        
        // キャッシュから取得
        let value: Option<String> = self.conn.get(&key).await?;
        
        // 文字列からU256に変換
        match value {
            Some(s) => match U256::from_dec_str(&s) {
                Ok(hf) => Ok(Some(hf)),
                Err(e) => {
                    error!("ヘルスファクターの変換エラー: {} - {}", s, e);
                    Ok(None)
                }
            },
            None => Ok(None),
        }
    }
    
    /// 複数の借り手のヘルスファクターをバッチで取得
    pub async fn get_health_factors_batch(&mut self, borrowers: &[Address]) -> Result<HashMap<Address, Option<U256>>> {
        if borrowers.is_empty() {
            return Ok(HashMap::new());
        }
        
        let mut result = HashMap::with_capacity(borrowers.len());
        let mut pipe = redis::pipe();
        
        // すべてのキーを取得リクエストに追加
        for borrower in borrowers {
            let key = format!("{}{:?}", self.hf_prefix, borrower);
            pipe.get(&key);
        }
        
        // パイプラインを実行して結果を取得
        let values: Vec<Option<String>> = pipe.query_async(&mut self.conn).await?;
        
        // 結果を整形
        for (i, borrower) in borrowers.iter().enumerate() {
            if let Some(value) = &values[i] {
                match U256::from_dec_str(value) {
                    Ok(hf) => result.insert(*borrower, Some(hf)),
                    Err(_) => result.insert(*borrower, None),
                };
            } else {
                result.insert(*borrower, None);
            }
        }
        
        Ok(result)
    }
    
    /// 借り手の最終スキャン時刻を記録
    pub async fn record_last_scan(&mut self, borrower: &Address) -> Result<()> {
        let key = format!("{}{:?}", self.last_scan_prefix, borrower);
        let now = Utc::now().to_rfc3339();
        
        // 1日間保持
        let _: () = self.conn.set_ex(&key, now, 86400).await?;
        
        Ok(())
    }
    
    /// 借り手の最終スキャン時刻を取得
    pub async fn get_last_scan(&mut self, borrower: &Address) -> Result<Option<DateTime<Utc>>> {
        let key = format!("{}{:?}", self.last_scan_prefix, borrower);
        
        // キャッシュから取得
        let value: Option<String> = self.conn.get(&key).await?;
        
        // 文字列からDateTimeに変換
        match value {
            Some(s) => match DateTime::parse_from_rfc3339(&s) {
                Ok(dt) => Ok(Some(dt.with_timezone(&Utc))),
                Err(e) => {
                    error!("日時の変換エラー: {} - {}", s, e);
                    Ok(None)
                }
            },
            None => Ok(None),
        }
    }
    
    /// 借り手の変化率を記録
    pub async fn cache_change_rate(&mut self, borrower: &Address, change_rate: f64) -> Result<()> {
        let key = format!("{}{:?}", self.change_rate_prefix, borrower);
        
        // 1時間保持
        let _: () = self.conn.set_ex(&key, change_rate.to_string(), 3600).await?;
        
        Ok(())
    }
    
    /// 借り手の変化率を取得
    pub async fn get_change_rate(&mut self, borrower: &Address) -> Result<Option<f64>> {
        let key = format!("{}{:?}", self.change_rate_prefix, borrower);
        
        // キャッシュから取得
        let value: Option<String> = self.conn.get(&key).await?;
        
        // 文字列からf64に変換
        match value {
            Some(s) => match s.parse::<f64>() {
                Ok(rate) => Ok(Some(rate)),
                Err(e) => {
                    error!("変化率の変換エラー: {} - {}", s, e);
                    Ok(None)
                }
            },
            None => Ok(None),
        }
    }
    
    /// 優先度の高い（ヘルスファクターの低い）借り手を取得
    pub async fn get_highest_priority(&mut self, count: isize) -> Result<Vec<(Address, f64)>> {
        // ZRANGE key 0 count-1 WITHSCORES
        // スコアが低い順（ヘルスファクターが低い順）に取得
        let results: Vec<(String, f64)> = redis::cmd("ZRANGE")
            .arg(&self.priority_queue_key)
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
    
    /// 特定のスコア範囲内の借り手数をカウント
    pub async fn count_in_range(&mut self, min_score: f64, max_score: f64) -> Result<usize> {
        // ZCOUNT key min max
        let count: usize = redis::cmd("ZCOUNT")
            .arg(&self.priority_queue_key)
            .arg(min_score)
            .arg(max_score)
            .query_async(&mut self.conn)
            .await?;
        
        Ok(count)
    }
    
    /// 優先度キュー内の借り手数を取得
    pub async fn size(&mut self) -> Result<usize> {
        // ZCARD key
        let size: usize = redis::cmd("ZCARD")
            .arg(&self.priority_queue_key)
            .query_async(&mut self.conn)
            .await?;
        
        Ok(size)
    }
    
    /// 借り手を優先度キューから削除
    pub async fn remove_borrower(&mut self, borrower: &Address) -> Result<()> {
        // ZREM key member
        let _: () = redis::cmd("ZREM")
            .arg(&self.priority_queue_key)
            .arg(borrower.to_string())
            .query_async(&mut self.conn)
            .await?;
        
        // ヘルスファクターと変化率のキャッシュも削除
        let hf_key = format!("{}{:?}", self.hf_prefix, borrower);
        let cr_key = format!("{}{:?}", self.change_rate_prefix, borrower);
        
        let mut pipe = redis::pipe();
        pipe.del(&hf_key);
        pipe.del(&cr_key);
        
        let _: () = pipe.query_async(&mut self.conn).await?;
        
        Ok(())
    }
    
    /// 借り手の優先度（スコア）を取得
    pub async fn get_borrower_priority(&mut self, borrower: &Address) -> Result<Option<f64>> {
        // ZSCORE key member
        let score: Option<f64> = redis::cmd("ZSCORE")
            .arg(&self.priority_queue_key)
            .arg(borrower.to_string())
            .query_async(&mut self.conn)
            .await?;
        
        Ok(score)
    }
    
    /// 古いエントリを削除
    pub async fn clear_old_entries(&mut self) -> Result<()> {
        // キャッシュエントリはTTLで自動的に期限切れになるので特別な処理は不要
        // ただし、優先度キューは自動的に期限切れにならないので、無効なエントリを削除
        
        // 優先度キュー内のすべての借り手を取得
        let all_borrowers: Vec<(String, f64)> = redis::cmd("ZRANGE")
            .arg(&self.priority_queue_key)
            .arg(0)
            .arg(-1)
            .arg("WITHSCORES")
            .query_async(&mut self.conn)
            .await?;
        
        if all_borrowers.is_empty() {
            return Ok(());
        }
        
        let mut to_remove = Vec::new();
        
        // 各借り手のヘルスファクターキャッシュが存在するかチェック
        for (addr_str, _) in all_borrowers {
            // アドレスを解析
            match addr_str.parse::<Address>() {
                Ok(addr) => {
                    let hf_key = format!("{}{:?}", self.hf_prefix, addr);
                    let exists: bool = self.conn.exists(&hf_key).await?;
                    
                    if !exists {
                        // キャッシュが存在しない場合は優先度キューからも削除
                        to_remove.push(addr_str);
                    }
                },
                Err(_) => {
                    // 無効なアドレス形式は削除
                    to_remove.push(addr_str);
                }
            }
        }
        
        // 削除対象があれば優先度キューから削除
        if !to_remove.is_empty() {
            let args: Vec<&str> = to_remove.iter().map(|s| s.as_str()).collect();
            let _: () = redis::cmd("ZREM")
                .arg(&self.priority_queue_key)
                .arg(args)
                .query_async(&mut self.conn)
                .await?;
            
            info!("優先度キューから{}件の古いエントリを削除しました", to_remove.len());
        }
        
        Ok(())
    }
    
    /// キーを完全削除（主にテスト用）
    pub async fn clear_all(&mut self) -> Result<()> {
        // HFキャッシュをクリア
        let pattern = format!("{}*", self.hf_prefix);
        let keys: Vec<String> = self.conn.keys(&pattern).await?;
        if !keys.is_empty() {
            let _: () = self.conn.del(keys).await?;
        }
        
        // 最終スキャン時刻をクリア
        let pattern = format!("{}*", self.last_scan_prefix);
        let keys: Vec<String> = self.conn.keys(&pattern).await?;
        if !keys.is_empty() {
            let _: () = self.conn.del(keys).await?;
        }
        
        // 変化率キャッシュをクリア
        let pattern = format!("{}*", self.change_rate_prefix);
        let keys: Vec<String> = self.conn.keys(&pattern).await?;
        if !keys.is_empty() {
            let _: () = self.conn.del(keys).await?;
        }
        
        // 優先度キューをクリア
        let _: () = self.conn.del(&self.priority_queue_key).await?;
        
        Ok(())
    }
    
    /// キャッシュの統計情報を取得
    pub async fn get_stats(&mut self) -> Result<(usize, usize, usize)> {
        // HFキャッシュのキー数
        let hf_pattern = format!("{}*", self.hf_prefix);
        let hf_keys: Vec<String> = self.conn.keys(&hf_pattern).await?;
        
        // 最終スキャン時刻のキー数
        let scan_pattern = format!("{}*", self.last_scan_prefix);
        let scan_keys: Vec<String> = self.conn.keys(&scan_pattern).await?;
        
        // 優先度キュー内の借り手数
        let queue_size: usize = self.size().await?;
        
        Ok((hf_keys.len(), scan_keys.len(), queue_size))
    }
    
    /// ヘルスファクターに基づいて適切なTTL（有効期限）を決定
    fn get_ttl_for_health_factor(&self, health_factor: &U256) -> Duration {
        // HFを1.0を基準とした倍率に変換
        let one_eth = U256::from_dec_str("1000000000000000000").unwrap(); // 1 ETH = 10^18
        
        if health_factor < &one_eth {
            // HF < 1.0: 30秒
            Duration::from_secs(30)
        } else if health_factor < &(one_eth + one_eth / 10) {
            // 1.0 <= HF < 1.1: 1分
            Duration::from_secs(60)
        } else if health_factor < &(one_eth + one_eth / 5) {
            // 1.1 <= HF < 1.2: 5分
            Duration::from_secs(300)
        } else if health_factor < &(one_eth + one_eth / 2) {
            // 1.2 <= HF < 1.5: 15分
            Duration::from_secs(900)
        } else if health_factor < &(one_eth * 2) {
            // 1.5 <= HF < 2.0: 30分
            Duration::from_secs(1800)
        } else {
            // HF >= 2.0: 1時間
            Duration::from_secs(3600)
        }
    }
} 