use base64::{engine::general_purpose, Engine as _};

/// 转码标准格式
#[inline]
pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
    general_purpose::STANDARD.encode(input.as_ref())
}

/// 解码标准格式
#[inline]
pub fn decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, base64::DecodeError> {
    general_purpose::STANDARD.decode(input.as_ref())
}
