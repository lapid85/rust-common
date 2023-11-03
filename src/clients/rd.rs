use crate::types::{Rd, ClusterRd};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::config::SITE_REDIS_STRINGS;

lazy_static! {
    pub static ref SERVERS: Arc<RwLock<HashMap<String, Rd>>> = Arc::new(RwLock::new(HashMap::new()));
    pub static ref CLUSTERS: Arc<RwLock<HashMap<String, ClusterRd>>> = Arc::new(RwLock::new(HashMap::new()));
}

/// 得到系统平台 redis client
pub async fn get(conn_string: &str) -> Rd {
    match Rd::open(conn_string) {
        Ok(client) => client,
        Err(e) => {
            panic!("get redis client error: {:?}", e);
        }
    }
}

/// 设置系统平台 redis client
pub async fn set(site: &str, conn_string: &str) {
    let mut servers = SERVERS.write().unwrap();
    let server = get(conn_string).await;
    (*servers).insert(site.to_owned(), server.clone());
}

/// 得到系统平台 redis client - 通过站点
pub async fn get_by_site(site: &str) -> Result<Rd, String> {
    let read_servers = match SERVERS.read() { 
        Ok(v) => v,
        Err(e) => {
            return Err(format!("get redis client error: {:?}", e));
        }
    };
    if let Some(server) = read_servers.get(site) {
        return Ok(server.clone());
    }
    // 从配置文件中读取
    let server_string = match SITE_REDIS_STRINGS.read() { 
        Ok(v) => v,
        Err(e) => {
            return Err(format!("get redis client error: {:?}", e));
        }
    };
    let Some(conn_string) = server_string.get(site)  else { 
        return Err("get redis string error".to_owned());
    };
    let mut servers = match SERVERS.write() { 
        Ok(v) => v,
        Err(e) => {
            return Err(format!("get redis client error: {:?}", e));
        }
    };
    let server = get(conn_string).await;
    (*servers).insert(site.to_owned(), server.clone());
    Ok(server)
}

/// 得到平台 redis cluster client - 通过站点
#[inline]
pub async fn get_by_request(req: &actix_web::HttpRequest) -> Result<Rd, String> {
    let site = crate::request::get_site_code(req);
    get_by_site(&site).await
}

/// 得到平台 redis cluster client
pub async fn get_cluster(nodes: Vec<String>) -> ClusterRd {
    ClusterRd::new(nodes).unwrap()
}

/// 设置平台 redis cluster client
pub async fn set_cluster(site: &str, nodes: Vec<String>) {
    let mut servers = CLUSTERS.write().unwrap();
    let server = get_cluster(nodes).await;
    (*servers).insert(site.to_owned(), server.clone());
}

/// 得到平台 redis cluster client - 通过站点
pub async fn get_cluster_by_site(site: &str) -> Result<ClusterRd, String> {
    let read_servers = match CLUSTERS.write() { 
        Ok(v) => v,
        Err(e) => {
            return Err(format!("get redis client error: {:?}", e));
        }
    };
    if let Some(server) = read_servers.get(site) {
        return Ok(server.clone());
    }
    // 从配置文件中读取
    let server_string = match SITE_REDIS_STRINGS.read() { 
        Ok(v) => v,
        Err(e) => {
            return Err(format!("get redis client error: {:?}", e));
        }
    };
    let Some(conn_string) = server_string.get(site)  else { 
        return Err("get redis string error".to_owned());
    };
    let mut servers = match CLUSTERS.write() { 
        Ok(v) => v,
        Err(e) => {
            return Err(format!("get redis client error: {:?}", e));
        }
    };
    let server = get_cluster(vec![conn_string.clone()]).await;
    (*servers).insert(site.to_owned(), server.clone());
    Ok(server)
}



/// 得到平台 redis cluster client - 通过站点
#[inline]
pub async fn get_cluster_by_request(req: &actix_web::HttpRequest) -> Result<ClusterRd, String> {
    let site = crate::request::get_site_code(req);
    get_cluster_by_site(&site).await
}