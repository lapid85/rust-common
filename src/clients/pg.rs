use crate::consts;
use crate::types::{Db, PoolOptions};

/// 得到数据库连接池 - 通过连接字符串
pub async fn get(conn_string: &str) -> Db {
    match PoolOptions::new()
        .max_connections(consts::PG_MAX_CONNECTIONS)
        .connect(&conn_string)
        .await
    {
        Ok(v) => v,
        Err(err) => {
            panic!("连接数据库失败 {:?}, conn string: '{}'", err, conn_string);
        }
    }
}