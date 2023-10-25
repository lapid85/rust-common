use serde::ser;
use serde::Serialize;

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

pub mod bytes;
pub mod resp;

pub use bytes::*;
pub use resp::*;