use actix_web::HttpResponse;
use serde::ser;
use serde_cbor::to_vec;
use super::{JsonOK, JsonOKMsg, JsonError, JsonResult};

/// 处理转换 cbor
#[inline]
fn cbor_response<T: ser::Serialize>(data: &T) -> HttpResponse {
    let encode = to_vec(data);
    HttpResponse::Ok()
        //.append_header(("Content-Disposition", "inline"))
        .append_header(("Content-Type", "application/cbor"))
        .body(encode.unwrap())
}

/// 返回200
#[inline]
pub fn cbor_ok() -> HttpResponse {
    cbor_response(&JsonOK { code: 0 })
}

/// 成功带上信息
#[inline]
pub fn cbor_ok_msg(msg: &str) -> HttpResponse {
    cbor_response(&JsonOKMsg {
        code: 0,
        message: msg,
    })
}

/// 拒絕訪問
#[inline]
pub fn cbor_deny() -> HttpResponse {
    cbor_response(&"deny")
}

/// 返回错误消息
#[inline]
pub fn cbor_error<T: AsRef<str>>(message: T) -> HttpResponse {
    cbor_response(&JsonError {
        code: 500,
        message: message.as_ref(),
    })
}

/// 返回数据
#[inline]
pub fn cbor_result<T: ser::Serialize>(result: &T) -> HttpResponse {
    cbor_response(&JsonResult {
        code: 0,
        data: result,
    })
}

