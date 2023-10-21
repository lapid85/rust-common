use actix_web::HttpRequest;
use crate::types::Cond;

pub trait Search {
    fn get_query_cond(_req: &HttpRequest) -> Option<Cond> { None }
    fn get_cond(_req: &HttpRequest) -> Option<Cond> { None }
}