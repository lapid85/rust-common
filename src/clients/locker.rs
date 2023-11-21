use super::rd;
use crate::request;
use crate::types::Rd;
use actix_web::HttpRequest;
use log::{debug, error};

pub struct Locker {
    pub redis: redis::Client,
    pub lock_key: &'static str,
}

impl Locker {
    /// 获取分布式锁
    pub async fn lock_by_request(
        req: &HttpRequest,
        cache_key: &'static str,
    ) -> Result<Self, &'static str> {
        use redis::AsyncCommands;

        let site_code = request::get_site_code(req);
        let Ok(client) = rd::get_by_site(&site_code).await else {
            return Err("errGetRedisClient");
        };

        let Ok(mut rd_conn) = client.get_async_connection().await else {
            return Err("errGetRedisConnection");
        };

        let Ok(val) = rd_conn.incr::<&'static str, i32, i32>(cache_key, 1).await else {
            return Err("errIncrRedisKey");
        };

        if val > 1 {
            return Err("errLock");
        }

        Ok(Self {
            redis: client,
            lock_key: cache_key,
        })
    }

    /// 获取分布式锁
    pub async fn lock_by_rd(rd: &Rd, cache_key: &'static str) -> Result<Self, &'static str> {
        use redis::AsyncCommands;

        let Ok(mut rd_conn) = rd.get_async_connection().await else {
            return Err("errGetRedisConnection");
        };

        let Ok(val) = rd_conn.incr::<&'static str, i32, i32>(cache_key, 1).await else {
            return Err("errIncrRedisKey");
        };

        if val > 1 {
            return Err("errLock");
        }

        Ok(Self {
            redis: rd.clone(),
            lock_key: cache_key,
        })
    }
}

impl Drop for Locker {
    fn drop(&mut self) {
        use redis::Commands;
        let mut rd_conn = match self.redis.get_connection() {
            Ok(conn) => conn,
            Err(err) => {
                error!("无法获取redis connection: {}", err);
                return;
            }
        };

        let result = rd_conn.del::<&'static str, i32>(self.lock_key);
        debug!("释放分布式锁: {:?}", result);
    }
}
