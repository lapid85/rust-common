use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use log::error;
use crate::utils::dt;

/// token secret
const TOKEN_SECRET: &'static str = "qwe123QWE!@#";

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub username: String,
    pub ip: String,
    pub exp: i64,
}

/// 创建claims
pub fn new_claims(id: i32, username: String, ip: String) -> Claims {
    Claims {
        id,
        username,
        ip,
        exp: dt::timestamp(),
    }
}

/// 得到token
pub fn get_token(info: &Claims) -> Result<String, String> {
    match encode(&Header::default(), &info, &EncodingKey::from_secret(TOKEN_SECRET.as_ref())) {
        Ok(t) => Ok(t),
        Err(_) => Err("get token error".to_owned()),
    }
}

/// 检测token
pub fn get_claims(token: &str) -> Result<Claims, String> {
    let token_data = match decode::<Claims>(token, &DecodingKey::from_secret(TOKEN_SECRET.as_ref()), &Validation::default()) {
        Ok(c) => c,
        Err(err) => {
            error!("check token error: {}", err);
            return Err("check token error".to_owned());
        }
    };
    Ok(token_data.claims)
}