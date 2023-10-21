use actix_web::HttpRequest;
use crate::consts;
use std::collections::HashMap;

pub trait Pagination {
    fn split(req: &HttpRequest) -> HashMap<&str, &str> {
        let query_str = req.query_string();
        let mut query_map = HashMap::new();
        for item in query_str.split("&") {
            let mut pair = item.split("=");
            let key = pair.next().unwrap_or("");
            let value = pair.next().unwrap_or("");
            query_map.insert(key, value);
        }
        query_map
    }
    fn page(&self, req: &HttpRequest) -> i32 {
        let query_map = Self::split(req);
        if let Some(v) = query_map.get("page") {
            if let Ok(v) = v.parse::<i32>() {
                return v;
            }
        }
        consts::PAGE_DEFAULT
    }
    fn page_size(&self, req: &HttpRequest) -> i32 {
        let query_map = Self::split(req);
        if let Some(v) = query_map.get("limit") {
            if let Ok(v) = v.parse::<i32>() {
                return v;
            }
        }
        consts::PAGE_SIZE
    }
}
