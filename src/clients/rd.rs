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
pub async fn get_by_site(site: &str) -> Rd {
    let mut servers = SERVERS.lock().unwrap();
    if let Some(server) = servers.get(site) {
        return server.clone();
    }

    let server_string = SITE_REDIS_STRINGS.lock().unwrap();
    let conn_string = server_string.get(site).unwrap();
    let server = get(conn_string).await;
    (*servers).insert(site.to_owned(), server.clone());
    server
}