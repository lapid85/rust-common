use crate::consts;
use crate::types::{Db, PoolOptions};
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use crate::config::SITE_PGSQL_STRINGS;

lazy_static! {
    pub static ref SERVERS: Arc<Mutex<HashMap<String, Db>>> = Arc::new(Mutex::new(HashMap::new()));
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
    let mut servers = SERVERS.lock().unwrap();
    let server = get(conn_string).await;
    (*servers).insert(site.to_owned(), server.clone());
}

/// 得到数据库连接池 - 通过站点
pub async fn get_by_site(site: &str) -> Result<Db, String> {
    let mut servers = match SERVERS.lock() { 
        Ok(v) => v, 
        Err(err) => {
            return Err(format!("Error: get SERVERS lock {:?}", err));
        }
    };
    if let Some(server) = servers.get(site) {
        return Ok(server.clone());
    }
    let Ok(server_string) = SITE_PGSQL_STRINGS.lock() else {
        return Err("Error: get SITE_PGSQL_STRINGS lock".to_owned());
    };
    println!("site = {}, {:?}", site, server_string);
    let Some(conn_string) = server_string.get(site) else {
        return Err(format!("Error: get conn string by site '{}'", site));
    };
    let server = get(conn_string).await;
    (*servers).insert(site.to_owned(), server.clone());
    Ok(server)
}

/// 得到数据库连接池 - 通过请求
#[inline]
pub async fn get_by_request(req: &actix_web::HttpRequest) -> Result<Db, String> {
    let site = crate::request::get_site_code(req);
    get_by_site(&site).await
}
