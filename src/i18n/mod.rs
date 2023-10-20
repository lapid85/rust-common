use actix_web::HttpRequest;

/// 加载语言包
pub fn load() {
    rust_i18n::i18n!("locales");
}

/// Get I18n text
pub fn tr(req: &HttpRequest, key: &str) -> String {
    if let Some(language) = req.headers().get("Accept-Language") {
        if let Ok(v) = language.to_str() {
            return t!(key, locale = v);
        }
    }

    return "".to_owned();
}