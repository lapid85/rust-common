use actix_web::HttpRequest;

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
    let site = headers.get("site").unwrap();
    let site = site.to_str().unwrap();
    site.to_string()
}