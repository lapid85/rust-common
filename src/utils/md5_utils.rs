/// 安全密鑰
const SECRET_KEYS: &str = "!s@w4$qS%^(_123-=0Xha9452sLW^%sfa9)\\";

/// 对字符串进行 md5 加密
#[inline]
pub fn str(content: &str) -> String {
    let encrypt = md5::compute(content);
    format!("{:x}", encrypt)
}

/// 对字符串进行 md5 加密 - 加入安全密鑰
#[inline]
pub fn str_with_key(content: &str) -> String {
    let encrypt = md5::compute(format!("{}{}", content, SECRET_KEYS));
    format!("{:x}", encrypt)
}

/// 对字符串进行 md5 加密 - 加入安全密鑰
#[inline]
pub fn str_with(content: &str, key: &str) -> String {
    let encrypt = md5::compute(format!("{}{}{}", content, key, SECRET_KEYS));
    format!("{:x}", encrypt)
}

// 对用户名、密码、时间戳进行 md5 加密
pub fn password(username: &String, password: &String, timestamp: &i64) -> String { 
    let encrypt = md5::compute(format!("{}{}{}{}", username, password, timestamp, SECRET_KEYS));
    format!("{:x}", encrypt)
}
