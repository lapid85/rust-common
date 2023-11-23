use super::rd;
use crate::request;
use crate::types::Rd;
use actix_web::HttpRequest;
use log::{debug, error};
use std::borrow::Cow;

pub struct Locker<'a> {
    pub redis: Cow<'a, redis::Client>,
    pub lock_key: String,
}

/// 获取分布式锁
pub async fn lock_by_request<'a, T: std::fmt::Display>(
    req: &'a HttpRequest,
    path: &'static str,
    ext_info: &'a T,
) -> Result<Locker<'a>, &'static str> {
    use redis::AsyncCommands;

    let site_code = request::get_site_code(req);
    let Ok(client) = rd::get_by_site(&site_code).await else {
        return Err("errGetRedisClient");
    };

    let Ok(mut rd_conn) = client.get_async_connection().await else {
        return Err("errGetRedisConnection");
    };

    let cache_key = format!("{}:{}", path, ext_info);
    let Ok(val) = rd_conn.incr::<&String, i32, i32>(&cache_key, 1).await else {
        return Err("errIncrRedisKey");
    };

    if val > 1 {
        return Err("errLock");
    }

    Ok(Locker {
        redis: Cow::Owned(client),
        lock_key: cache_key,
    })
}

/// 获取分布式锁
pub async fn lock_by_rd<'a, T: std::fmt::Display>(
    rd: &'a Rd,
    path: &'static str,
    ext_info: &T,
) -> Result<Locker<'a>, &'static str> {
    use redis::AsyncCommands;

    let Ok(mut rd_conn) = rd.get_async_connection().await else {
        return Err("errGetRedisConnection");
    };

    let cache_key = format!("{}:{}", path, ext_info);
    let Ok(val) = rd_conn.incr::<&String, i32, i32>(&cache_key, 1).await else {
        return Err("errIncrRedisKey");
    };

    if val > 1 {
        return Err("errLock");
    }

    Ok(Locker {
        redis: Cow::Borrowed(rd),
        lock_key: cache_key,
    })
}

impl<'a> Drop for Locker<'a> {
    fn drop(&mut self) {
        use redis::Commands;
        let mut rd_conn = match self.redis.get_connection() {
            Ok(conn) => conn,
            Err(err) => {
                error!("无法获取redis connection: {}", err);
                return;
            }
        };

        let result = rd_conn.del::<&String, i32>(&self.lock_key);
        debug!("释放分布式锁: {:?}", result);
    }
}
