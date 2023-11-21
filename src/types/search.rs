use crate::types::Cond;
use actix_web::HttpRequest;

pub trait Search {
    fn get_query_cond(_req: &HttpRequest) -> Option<Cond> {
        None
    }
    fn get_cond(_req: &HttpRequest) -> Option<Cond> {
        None
    }
}
