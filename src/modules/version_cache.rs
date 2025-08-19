use deadpool_redis::redis::AsyncCommands;
use dashmap::DashMap;
use std::sync::Arc;
use std::collections::{HashMap, HashSet};
use crate::redis_pool::RedisPool;
use crate::entity::version_info::VersionInfo;

#[derive(Clone)]
pub struct VersionCache {
    pub version_map: Arc<DashMap<String, Arc<VersionInfo>>>, // 预解析 JSON
    pub version_last_map: Arc<DashMap<String,String>>,
    pub channel_set: Arc<DashMap<String, ()>>, // set 用 DashMap 当做 concurrent set
    pub device_white_set: Arc<DashMap<String, ()>>,
}

impl VersionCache {
    pub fn new() -> Self {
        Self {
            version_map: Arc::new(DashMap::new()),
            version_last_map: Arc::new(DashMap::new()),
            channel_set: Arc::new(DashMap::new()),
            device_white_set: Arc::new(DashMap::new()),
        }
    }

    /// 加载 Redis 数据并预解析 JSON
    pub async fn load_from_redis(&self, redis_pool: &RedisPool) -> Result<(), String> {
        let mut conn = redis_pool.get().await.map_err(|e| e.to_string())?;

        // gm:version (hash)
        let version_data: HashMap<String, String> =
            conn.hgetall("gm:version").await.map_err(|e| e.to_string())?;
        self.version_map.clear();
        for (k, v) in version_data {
            match serde_json::from_str::<VersionInfo>(&v) {
                Ok(info) => { self.version_map.insert(k, Arc::new(info)); },
                Err(e) => { log::error!("Parse VersionInfo failed: {}", e); }
            }
        }

        // gm:version:last (hash)
        let last_version_data: HashMap<String,String> =
            conn.hgetall("gm:version:last").await.map_err(|e| e.to_string())?;
        self.version_last_map.clear();
        for (k,v) in last_version_data { self.version_last_map.insert(k,v); }

        // gm:version:channel (set)
        let channel_data: HashSet<String> =
            conn.smembers("gm:version:channel").await.map_err(|e| e.to_string())?;
        self.channel_set.clear();
        for v in channel_data { self.channel_set.insert(v, ()); }

        // gm:version:device (set)
        let device_data: HashSet<String> =
            conn.smembers("gm:version:device").await.map_err(|e| e.to_string())?;
        self.device_white_set.clear();
        for v in device_data { self.device_white_set.insert(v, ()); }

        Ok(())
    }

    /// 获取 VersionInfo，避免每次 clone
    pub fn get_version_info(&self, key: &str) -> Option<Arc<VersionInfo>> {
        self.version_map.get(key).map(|v| v.clone())
    }

    pub fn is_device_white(&self, device_no: &str) -> bool {
        self.device_white_set.contains_key(device_no)
    }

    pub fn has_channel(&self, channel: &str) -> bool {
        self.channel_set.contains_key(channel)
    }
}
