use actix_web::HttpResponse;
use serde::ser;
use serde::Serialize;
use serde_cbor::to_vec;

/// 处理成功
#[derive(Serialize)]
pub struct JsonOK {
    pub code: u16,
}

/// 处理成功
#[derive(Serialize)]
pub struct JsonOKMsg<T: AsRef<str>> {
    pub code: u16,
    pub message: T,
}

/// 处理失败
#[derive(Serialize)]
pub struct JsonError<T: AsRef<str>> {
    pub code: u16,
    pub message: T,
}

/// 返回结果
#[derive(Serialize)]
pub struct JsonResult<T: ser::Serialize> {
    pub code: u16,
    pub data: T,
}

/// 返回200
pub fn ok() -> HttpResponse {
    HttpResponse::Ok().json(JsonOK { code: 0 })
}

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

/// 返回200
#[inline]
pub fn ok_msg(msg: &str) -> HttpResponse {
    HttpResponse::Ok().json(JsonOKMsg {
        code: 0,
        message: msg,
    })
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
pub fn deny() -> HttpResponse {
    HttpResponse::Ok().body("deny")
}

/// 拒絕訪問
#[inline]
pub fn cbor_deny() -> HttpResponse {
    cbor_response(&"deny")
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
pub fn result<T: ser::Serialize>(result: &T) -> HttpResponse {
    let res = JsonResult {
        code: 0,
        data: result,
    };
    HttpResponse::Ok().json(res)
}

/// 返回数据
#[inline]
pub fn cbor_result<T: ser::Serialize>(result: &T) -> HttpResponse {
    cbor_response(&JsonResult {
        code: 0,
        data: result,
    })
}
