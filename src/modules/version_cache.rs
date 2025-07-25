use deadpool_redis::redis::AsyncCommands;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::redis_pool::RedisPool;

#[derive(Clone, Debug)]
pub struct VersionCache {
    // 用于存储 Hash 类型的数据，例如 gm:version
    pub version_map: Arc<RwLock<HashMap<String, String>>>,

    // 用于存储 Set 类型的数据，例如 gm:version:channel
    pub channel_set: Arc<RwLock<HashSet<String>>>,
}

impl VersionCache {
    pub fn new() -> Self {
        Self {
            version_map: Arc::new(RwLock::new(HashMap::new())),
            channel_set: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    /// 加载 Redis 中 gm:version（hash）和 gm:version:channel（set）数据
    pub async fn load_from_redis(&self, redis_pool: &RedisPool) -> Result<(), String> {
        let mut conn = redis_pool.get().await.map_err(|e| e.to_string())?;

        // 加载 gm:version (hash)
        let version_data: HashMap<String, String> =
            conn.hgetall("gm:version").await.map_err(|e| e.to_string())?;
        {
            let mut map_guard = self.version_map.write().await;
            *map_guard = version_data;
        }

        // 加载 gm:version:channel (set)
        let channel_data: HashSet<String> =
            conn.smembers("gm:version:channel").await.map_err(|e| e.to_string())?;
        {
            let mut set_guard = self.channel_set.write().await;
            *set_guard = channel_data;
        }

        Ok(())
    }

    /// 获取 Hash 数据
    pub async fn get_version_map(&self) -> HashMap<String, String> {
        self.version_map.read().await.clone()
    }

    /// 获取 Set 数据
    pub async fn get_channel_set(&self) -> HashSet<String> {
        self.channel_set.read().await.clone()
    }

    /// 手动刷新
    pub async fn refresh(&self, redis_pool: &RedisPool) -> Result<(), String> {
        self.load_from_redis(redis_pool).await
    }
}
