use crate::types::{Rd, ClusterRd};

/// 得到系统平台 redis client
pub async fn get(conn_string: &str) -> Rd {
    match Client::open(conn_string) {
        Ok(client) => client,
        Err(e) => {
            panic!("get redis client error: {:?}", e);
        }
    }
}

/// 得到平台 redis cluster client
pub async fn get_cluster(nodes: Vec<String>) -> ClusterRd {
    ClusterClient::new(nodes).unwrap()
}