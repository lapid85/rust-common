use crate::config::SITE_PGSQL_STRINGS;
use crate::consts;
use crate::types::{Db, PoolOptions};
use log::error;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

lazy_static! {
    pub static ref SERVERS: Arc<RwLock<HashMap<String, Db>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

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

/// 设置数据库连接池 - 通过站点
pub async fn set(site: &str, conn_string: &str) {
    let mut servers = SERVERS.write().unwrap();
    let server = get(conn_string).await;
    (*servers).insert(site.to_owned(), server.clone());
}

/// 得到数据库连接池 - 通过站点
pub async fn get_by_site(site: &str) -> Result<Db, &'static str> {
    {
        let Ok(read_servers) = SERVERS.read() else {
            return Err("Error: get SERVERS lock");
        };
        if let Some(server) = read_servers.get(site) {
            error!("get server from SERVERS read()");
            return Ok(server.clone());
        }
    }
    // 从配置文件中读取
    let Ok(server_string) = SITE_PGSQL_STRINGS.read() else {
        return Err("Error: get SITE_PGSQL_STRINGS lock");
    };
    let Some(conn_string) = server_string.get(site) else {
        error!("Error: get conn string by site");
        return Err("Error: get conn string by site");
    };
    {
        let Ok(mut servers) = SERVERS.write() else {
            error!("Error: get SERVERS lock");
            return Err("Error: get SERVERS lock");
        };
        let server = get(conn_string).await;
        (*servers).insert(site.to_owned(), server.clone());
        return Ok(server);
    }
}

/// 得到数据库连接池 - 通过请求
#[inline]
pub async fn get_by_request(req: &actix_web::HttpRequest) -> Result<Db, &'static str> {
    let site = crate::request::get_site_code(req);
    get_by_site(&site).await
}
