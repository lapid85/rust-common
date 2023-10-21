use actix_web::HttpRequest;
use crate::consts;
use super::Cond;

pub trait Pagination {
    /// 获取分页参数
    fn get_limits(&self, req: &HttpRequest) -> (i32, i32) {
        let mut page = consts::PAGE_DEFAULT;
        let mut page_size = consts::PAGE_SIZE;

        let query_str = req.query_string();
        for item in query_str.split("&") {
            let mut pair = item.split("=");
            let key = pair.next().unwrap_or("");
            if key == "page" {
                if let Some(v) = pair.next() {
                    if let Ok(v) = v.parse::<i32>() {
                        page = v;
                    }
                }
                continue;
            }
            if key == "limit" {
                if let Some(v) = pair.next() {
                    if let Ok(v) = v.parse::<i32>() {
                        page_size = v;
                    }
                }
                continue;
            }
        }
        (page, page_size)
    }

    /// 获取分页条件
    fn get_cond(&self, req: &HttpRequest) -> Cond {
        let (page, page_size) = self.get_limits(req);
        let mut cond = Cond::new();
        cond.page(page).limit(page_size);
        cond
    }
}
