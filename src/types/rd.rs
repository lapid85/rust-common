// use futures::prelude::*;
// use redis::AsyncCommands;
// let client = redis::Client::open("redis://127.0.0.1/").unwrap();
// let mut con = client.get_async_connection().await?;
pub type Conn = redis::aio::Connection;
pub type Client = redis::Client;
pub type ClusterClient = redis::cluster::ClusterClient;
pub type ClusterConn = redis::cluster_async::ClusterConnection;
// pub type Result<T> = Result<T, &'static str>;