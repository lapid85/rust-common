use crate::consts::{AUTHORIZATION, PLATFORM_SYSTEM};
use actix_web::HttpRequest;

/// 获取ip
const IP_HEADERS: [&'static str; 7] = [
    "x-real-ip",
    "x-forwarded-for",
    "x-client-ip",
    "x-cluster-client-ip",
    "forwarded-for",
    "client-ip",
    "remote-addr",
];

/// 获取 GET 请求结果
pub async fn get(url: &str) -> Result<String, &'static str> {
    let Ok(response) = reqwest::get(url).await else {
        return Err("发送请求失败");
    };
    let Ok(body) = response.text().await else {
        return Err("获取结果失败");
    };
    Ok(body)
}

/// 获取 POST 请求结果
pub async fn post(url: &str, body: &str) -> Result<String, &'static str> {
    let Ok(response) = reqwest::Client::new()
        .post(url)
        .body(body.to_string())
        .send()
        .await
    else {
        return Err("发送请求失败");
    };
    let Ok(body) = response.text().await else {
        return Err("获取结果失败");
    };
    Ok(body)
}

/// 获取 PUT 请求结果
pub async fn put(url: &str, body: &str) -> Result<String, &'static str> {
    let Ok(response) = reqwest::Client::new()
        .put(url)
        .body(body.to_string())
        .send()
        .await
    else {
        return Err("发送请求失败");
    };
    let Ok(body) = response.text().await else {
        return Err("获取结果失败");
    };
    Ok(body)
}

/// 获取 DELETE 请求结果
pub async fn delete(url: &str, body: &str) -> Result<String, &'static str> {
    let Ok(response) = reqwest::Client::new()
        .delete(url)
        .body(body.to_string())
        .send()
        .await
    else {
        return Err("发送请求失败");
    };
    let Ok(body) = response.text().await else {
        return Err("获取结果失败");
    };
    Ok(body)
}

/// 获取站点代码
pub fn get_site_code(req: &HttpRequest) -> String {
    let headers = req.headers();
    if let Some(site) = headers.get("site") {
        let site = site.to_str().unwrap();
        return site.to_uppercase();
    }

    let path = req.path();
    if path.contains("/plat/") {
        // 如果包含 /plat/ 则为平台
        return PLATFORM_SYSTEM.to_owned();
    }

    "AK".to_owned()
}

/// 获取ip
pub fn client_ip(req: &HttpRequest) -> Option<String> {
    let headers = req.headers();
    for h in IP_HEADERS {
        if let Some(ip) = headers.get(h) {
            let ip = ip.to_str().unwrap();
            return Some(ip.to_owned());
        }
    }

    None
}

/// 获取token
pub fn get_token_str<'a>(req: &'a HttpRequest) -> Result<&'a str, &'static str> {
    let headers = req.headers();
    let Some(token_val) = headers.get(AUTHORIZATION) else {
        return Err("get token error");
    };
    let Ok(token_str) = token_val.to_str() else {
        return Err("get token error");
    };
    Ok(token_str)
}
