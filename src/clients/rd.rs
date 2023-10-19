use crate::types::{Rd, ClusterRd};
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use crate::config::SITE_REDIS_STRINGS;

lazy_static! {
    pub static ref SERVERS: Arc<Mutex<HashMap<String, Rd>>> = Arc::new(Mutex::new(HashMap::new()));
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

/// 得到平台 redis cluster client
pub async fn get_cluster(nodes: Vec<String>) -> ClusterRd {
    ClusterRd::new(nodes).unwrap()
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