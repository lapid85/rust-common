use crate::clients;
use crate::consts::PLATFORM_SYSTEM;
use dotenv;
use log::info;
use sqlx::FromRow;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// 获得配置内容
pub mod env;

lazy_static! {
    pub static ref SITE_STATIC_URLS: Arc<RwLock<HashMap<String, String>>> =
        Arc::new(RwLock::new(HashMap::new()));
    pub static ref SITE_PGSQL_STRINGS: Arc<RwLock<HashMap<String, String>>> =
        Arc::new(RwLock::new(HashMap::new()));
    pub static ref SITE_REDIS_STRINGS: Arc<RwLock<HashMap<String, String>>> =
        Arc::new(RwLock::new(HashMap::new()));
    pub static ref SITE_REDIS_CLUSTER: Arc<RwLock<HashMap<String, String>>> =
        Arc::new(RwLock::new(HashMap::new()));
    pub static ref SITE_KAFKA_STRINGS: Arc<RwLock<HashMap<String, String>>> =
        Arc::new(RwLock::new(HashMap::new()));
    pub static ref SITE_NAMES: Arc<RwLock<HashMap<String, String>>> =
        Arc::new(RwLock::new(HashMap::new()));
    pub static ref SITE_PLATFORMS: Arc<RwLock<HashMap<String, String>>> =
        Arc::new(RwLock::new(HashMap::new()));
    pub static ref SITE_UP_URLS: Arc<RwLock<HashMap<String, String>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

// 加载配置文件
pub fn load_env() {
    dotenv::dotenv().ok();
}

#[derive(Debug, FromRow)]
pub struct Platform {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, FromRow)]
pub struct Site {
    pub id: i32,
    pub name: String,
    pub platform_id: i32,
    pub platform_name: String,
    pub code: String,
}

#[derive(Debug, FromRow)]
pub struct Config {
    pub id: i32,
    pub name: String,
    pub platform_id: i32,
    pub platform_name: String,
    pub site_id: i32,
    pub site_name: String,
    pub value: String,
}

/// 设置配置内容
pub async fn load_all(conn_string: &str) {
    let db = clients::pg::get(conn_string).await;
    let platforms: Vec<Platform> =
        sqlx::query_as("select id, name from platforms where status = 1")
            .fetch_all(&db)
            .await
            .unwrap();

    let mut site_static_urls = SITE_STATIC_URLS.write().unwrap();
    let mut site_pgsql_strings = SITE_PGSQL_STRINGS.write().unwrap();
    let mut site_redis_strings = SITE_REDIS_STRINGS.write().unwrap();
    let mut site_redis_cluster = SITE_REDIS_CLUSTER.write().unwrap();
    let mut site_kafka_strings = SITE_KAFKA_STRINGS.write().unwrap();
    let mut site_names = SITE_NAMES.write().unwrap();
    let mut site_platforms = SITE_PLATFORMS.write().unwrap();
    let mut site_up_urls = SITE_UP_URLS.write().unwrap();

    // 设置默认的数据库连接字符串
    site_pgsql_strings.insert(PLATFORM_SYSTEM.to_owned(), conn_string.to_owned());

    for platform in platforms {
        let sites: Vec<Site> = sqlx::query_as("select id, name, platform_id, platform_name, code from sites where status = 1 AND platform_id = $1")
            .bind(platform.id)
            .fetch_all(&db)
            .await
            .unwrap();
        for site in sites {
            let code = site.code.clone();
            site_platforms.insert(code.clone(), platform.name.clone());
            site_names.insert(code.clone(), site.name.clone());

            let configs: Vec<Config> = sqlx::query_as("select id, name, platform_id, platform_name, site_id, site_name, value from configs where status = 1 AND platform_id = $1 and site_id = $2")
                .bind(platform.id)
                .bind(site.id)
                .fetch_all(&db)
                .await
                .unwrap();
            for config in configs {
                if config.name == "pgsql_strings" {
                    site_pgsql_strings.insert(code.clone(), config.value.clone());
                } else if config.name == "redis_strings" {
                    site_redis_strings.insert(code.clone(), config.value.clone());
                } else if config.name == "redis_cluster_strings" {
                    site_redis_cluster.insert(code.clone(), config.value.clone());
                } else if config.name == "kafka_strings" {
                    site_kafka_strings.insert(code.clone(), config.value.clone());
                } else if config.name == "static_url" {
                    site_static_urls.insert(code.clone(), config.value.clone());
                } else if config.name == "up_url" {
                    site_up_urls.insert(code.clone(), config.value.clone());
                }
            }
        }
    }

    info!("site_platforms = {:?}", site_platforms);
    info!("site_pgsql_strings = {:?}", site_pgsql_strings);
    info!("site_redis_strings = {:?}", site_redis_strings);
    info!("site_redis_cluster = {:?}", site_redis_cluster);
    info!("site_kafka_strings = {:?}", site_kafka_strings);
    info!("site_static_urls = {:?}", site_static_urls);
    info!("site_up_urls = {:?}", site_up_urls);
    info!("site_names = {:?}", site_names);
}
