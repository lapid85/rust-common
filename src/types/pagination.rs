use actix_web::HttpRequest;
use crate::consts;
use super::Cond;

pub trait Pagination {
    /// 获取分页参数
    fn get_limits(&self, req: &HttpRequest) -> (i32, i32) {
        let mut page = consts::PAGE_DEFAULT;
        let mut page_size = consts::PAGE_SIZE;

        let query_str = req.query_string();
        println!("query_str: {}", query_str);
        for item in query_str.split("&") {
            let pair = item.split("=").collect::<Vec<&str>>();
            println!("pair: {:?}", pair);
            let key = pair[0];
            let value = pair[1];
            println!("key: {}, value: {}", key, value);
            if key == "page" {
                if let Ok(v) = value.parse::<i32>() {
                    page = v;
                }
                continue;
            }
            if key == "limit" {
                if let Ok(v) = value.parse::<i32>() {
                    page_size = v;
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
