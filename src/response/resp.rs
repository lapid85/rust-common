use actix_web::HttpResponse;
use serde::ser;
use super::{JsonOK, JsonOKMsg, JsonError, JsonResult};
use log::error;

/// 返回200
pub fn ok() -> HttpResponse {
    #[cfg(not(feature = "response_cbor"))]
    return HttpResponse::Ok().json(JsonOK { code: 0 });
    #[cfg(feature = "response_cbor")]
    return super::cbor_response(&JsonOK { code: 0 });
}


/// 返回200
#[inline]
pub fn ok_msg(msg: &str) -> HttpResponse {
    #[cfg(not(feature = "response_cbor"))]
    return HttpResponse::Ok().json(JsonOKMsg {
        code: 0, 
        message: msg,
    });
    #[cfg(feature = "response_cbor")]
    return super::cbor_response(&JsonOKMsg {
        code: 0,
        message: msg,
    });
}

/// 拒絕訪問
#[inline]
pub fn deny() -> HttpResponse {
    #[cfg(not(feature = "response_cbor"))]
    return HttpResponse::Ok().body("deny");
    #[cfg(feature = "response_cbor")]
    return super::cbor_response(&"deny");
}

/// 返回错误消息
#[inline]
pub fn error<T: AsRef<str>>(message: T) -> HttpResponse {
    error!("返回错误信息: {}", message.as_ref());
    #[cfg(not(feature = "response_cbor"))]
    return HttpResponse::Ok().json(&JsonError {
        code: 500, 
        message: message.as_ref(),
    });
    #[cfg(feature = "response_cbor")]
    return super::cbor_response(&JsonError {
        code: 500,
        message: message.as_ref(),
    });
}

/// 返回数据
#[inline]
pub fn result<T: ser::Serialize>(result: &T) -> HttpResponse {
    #[cfg(not(feature = "response_cbor"))]
    return HttpResponse::Ok().json(&JsonResult {
        code: 0,
        data: result,
    });
    #[cfg(feature = "response_cbor")]
    return super::cbor_response(&JsonResult {
        code: 0,
        data: result,
    });
}