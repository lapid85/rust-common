use reqwest::{Client, ClientBuilder};

const USER_AGENT: &'static str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36";

/// 获取http client 客户端
pub async fn get() -> Client {
    ClientBuilder::new().user_agent(USER_AGENT).build().unwrap()
}
