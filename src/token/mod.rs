use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use log::error;
use crate::utils::dt;
use actix_web::HttpRequest;
use std::collections::HashMap;
use std::sync::Mutex;

/// token secret
const TOKEN_SECRET: &'static str = "qwe123QWE!@#";
const TOKEN_EXPIRE: i64 = 1800; // token过期时间

lazy_static! {
    static ref USER_TOKENS: Mutex<HashMap<String, i64>> = Mutex::new(HashMap::new());
}

/// token
#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
    pub expires: i64,
}

/// 检查token是否过期
pub fn start() { 
    let expire_secs = tokio::time::Duration::from_secs(5); // 每5秒钟检查一次过期token
    tokio::task::spawn(async move {
        loop {
            tokio::time::sleep(expire_secs).await;
            let Ok(mut tokens) = USER_TOKENS.lock() else { 
                error!("get token lock error");
                continue;
            };
            let now = dt::timestamp();
            let mut del_keys = Vec::new();
            for (k, v) in tokens.iter() {
                if now - v > TOKEN_EXPIRE {
                    del_keys.push(k.clone());
                }
            }
            for k in del_keys {
                tokens.remove(&k);
            }
        }
    });
}

/// 添加token
pub fn add(token: &str, timestamp: i64) {
    let Ok(mut tokens) = USER_TOKENS.lock() else { 
        error!("get token lock error");
        return;
    };
    tokens.insert(token.to_owned(), timestamp);
}

/// 刷新token
pub fn refresh(token_old: &str, token_new: &str) {
    let Ok(mut tokens) = USER_TOKENS.lock() else { 
        error!("get token lock error");
        return;
    };
    tokens.remove(token_old);
    tokens.insert(token_new.to_owned(), dt::timestamp());
}

/// 添加token
pub fn has(token: &str) -> bool {
    let Ok(tokens) = USER_TOKENS.lock() else {
        error!("get token lock error");
        return false;
    };
    tokens.contains_key(token)
}

/// 添加token
pub fn remove(token: &str) {
    let Ok(mut tokens) = USER_TOKENS.lock() else {
        error!("get token lock error");
        return;
    };
    tokens.remove(token);
}

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub username: String,
    pub ip: String,
    pub exp: i64,
}

/// 创建claims
pub fn new_claims(id: i32, username: &String, ip: &String) -> Claims {
    Claims {
        id,
        username: username.clone(),
        ip: ip.clone(),
        exp: dt::timestamp(),
    }
}

/// 得到token
#[inline]
pub fn get_token(info: &Claims) -> Result<String, String> {
    match encode(&Header::default(), &info, &EncodingKey::from_secret(TOKEN_SECRET.as_ref())) {
        Ok(t) => Ok(t),
        Err(_) => Err("get token error".to_owned()),
    }
}

/// 检测token
#[inline]
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

/// 检测token
#[inline]
pub fn get_claims_by_request(req: &HttpRequest) -> Result<Claims, String> {
    let token_str = match match req.headers().get("Authorization") {
        Some(v) => v.to_str(),
        None => { 
            error!("get token from request error");
            return Err("get token from request error".to_owned());
        }
    }  {
        Ok(v) => v,
        Err(err) => {
            error!("get token from request error: {}", err);
            return Err("get token from request error".to_owned());
        }
    };
    get_claims(&token_str)
}