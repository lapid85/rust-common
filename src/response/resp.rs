use actix_web::HttpResponse;
use serde::ser;
use super::{JsonOK, JsonOKMsg, JsonError, JsonResult};

/// 返回200
pub fn ok() -> HttpResponse {
    HttpResponse::Ok().json(JsonOK { code: 0 })
}


/// 返回200
#[inline]
pub fn ok_msg(msg: &str) -> HttpResponse {
    HttpResponse::Ok().json(JsonOKMsg {
        code: 0,
        message: msg,
    })
}

/// 拒絕訪問
#[inline]
pub fn deny() -> HttpResponse {
    HttpResponse::Ok().body("deny")
}

/// 返回错误消息
#[inline]
pub fn error<T: AsRef<str>>(message: T) -> HttpResponse {
    let err = JsonError {
        code: 500,
        message: message.as_ref(),
    };
    HttpResponse::Ok().json(err)
}

/// 返回数据
#[inline]
pub fn result<T: ser::Serialize>(result: &T) -> HttpResponse {
    let res = JsonResult {
        code: 0,
        data: result,
    };
    HttpResponse::Ok().json(res)
}