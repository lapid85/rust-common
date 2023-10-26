use crate::types::{Rd, ClusterRd};
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use crate::config::SITE_REDIS_STRINGS;

lazy_static! {
    pub static ref SERVERS: Arc<Mutex<HashMap<String, Rd>>> = Arc::new(Mutex::new(HashMap::new()));
    pub static ref CLUSTERS: Arc<Mutex<HashMap<String, ClusterRd>>> = Arc::new(Mutex::new(HashMap::new()));
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
    let mut servers = SERVERS.lock().unwrap();
    let server = get(conn_string).await;
    (*servers).insert(site.to_owned(), server.clone());
}

/// 得到平台 redis cluster client
pub async fn get_cluster(nodes: Vec<String>) -> ClusterRd {
    ClusterRd::new(nodes).unwrap()
}

/// 设置平台 redis cluster client
pub async fn set_cluster(site: &str, nodes: Vec<String>) {
    let mut servers = CLUSTERS.lock().unwrap();
    let server = get_cluster(nodes).await;
    (*servers).insert(site.to_owned(), server.clone());
}

/// 得到系统平台 redis client - 通过站点
pub async fn get_by_site(site: &str) -> Result<Rd, String> {
    let mut servers = match SERVERS.lock() { 
        Ok(v) => v,
        Err(e) => {
            return Err(format!("get redis client error: {:?}", e));
        }
    };
    if let Some(server) = servers.get(site) {
        return Ok(server.clone());
    }
    let server_string = match SITE_REDIS_STRINGS.lock() { 
        Ok(v) => v,
        Err(e) => {
            return Err(format!("get redis client error: {:?}", e));
        }
    };
    let Some(conn_string) = server_string.get(site)  else { 
        return Err("get redis string error".to_owned());
    };
    let server = get(conn_string).await;
    (*servers).insert(site.to_owned(), server.clone());
    Ok(server)
}

/// 得到平台 redis cluster client - 通过站点
pub async fn get_cluster_by_site(site: &str) -> Result<ClusterRd, String> {
    let mut servers = match CLUSTERS.lock() { 
        Ok(v) => v,
        Err(e) => {
            return Err(format!("get redis client error: {:?}", e));
        }
    };
    if let Some(server) = servers.get(site) {
        return Ok(server.clone());
    }
    let server_string = match SITE_REDIS_STRINGS.lock() { 
        Ok(v) => v,
        Err(e) => {
            return Err(format!("get redis client error: {:?}", e));
        }
    };
    let Some(conn_string) = server_string.get(site)  else { 
        return Err("get redis string error".to_owned());
    };
    let server = get_cluster(vec![conn_string.clone()]).await;
    (*servers).insert(site.to_owned(), server.clone());
    Ok(server)
}

/// 得到平台 redis cluster client - 通过站点
#[inline]
pub async fn get_by_request(req: &actix_web::HttpRequest) -> Result<Rd, String> {
    let site = crate::request::get_site_code(req);
    get_by_site(&site).await
}

/// 得到平台 redis cluster client - 通过站点
#[inline]
pub async fn get_cluster_by_request(req: &actix_web::HttpRequest) -> Result<ClusterRd, String> {
    let site = crate::request::get_site_code(req);
    get_cluster_by_site(&site).await
}