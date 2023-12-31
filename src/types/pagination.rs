use super::Cond;
use crate::consts;
use actix_web::HttpRequest;

pub trait Pagination {
    /// 获取分页参数
    fn get_limits(&self, req: &HttpRequest) -> (i32, i32) {
        let mut page = consts::PAGE_DEFAULT;
        let mut page_size = consts::PAGE_SIZE;

        let query_str = req.query_string().trim();
        let arr = query_str.split("&").collect::<Vec<&str>>();
        if arr.len() == 0 {
            return (page, page_size);
        }
        for item in &arr {
            let pair = item.split("=").collect::<Vec<&str>>();
            if pair.len() != 2 {
                continue;
            }
            let key = pair[0];
            let value = pair[1];
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
