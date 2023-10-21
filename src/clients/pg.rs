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

/// 得到数据库连接池 - 通过站点
pub async fn get_by_site(site: &str) -> Db {
    let mut servers = SERVERS.lock().unwrap();
    if let Some(server) = servers.get(site) {
        return server.clone();
    }

    let server_string = SITE_PGSQL_STRINGS.lock().unwrap();
    let conn_string = server_string.get(site).unwrap();
    let server = get(conn_string).await;
    (*servers).insert(site.to_owned(), server.clone());
    server
}

/// 得到数据库连接池 - 通过请求
pub async fn get_by_request(req: &actix_web::HttpRequest) -> Db {
    let site = crate::request::get_site_code(req);
    get_by_site(&site).await
}
