use actix_cors::Cors;
use actix_web::http;

/// 允许跨域域名
pub fn allowed_domains(domains: &Vec<String>) -> actix_cors::Cors {
    let mut cors = Cors::default()
        .allowed_origin("http://localhost:8848")
        .allowed_origin("http://127.0.0.1:8080");

    for d in domains {
        if d.trim_matches(' ') == "" {
            continue;
        }
        cors = cors.allowed_origin(d);
    }

    cors.allowed_methods(vec!["GET", "POST", "OPTIONS"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
}
