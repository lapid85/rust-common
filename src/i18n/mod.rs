#[macro_export]
macro_rules! tr {
    ($req: expr, $key: expr) => {{
        if let Some(language) = $req.headers().get("Accept-Language") {
            if let Ok(v) = language.to_str() {
                match v { 
                    "zh-CN" | "zh-Hans" => rust_i18n::t!($key, locale = "zh-CN"),
                    "zh-TW" | "zh-Hant" => rust_i18n::t!($key, locale = "zh-TW"),
                    "en-US" | "en" => rust_i18n::t!($key, locale = "en"),
                    _ => rust_i18n::t!($key, locale = "zh-CN"),
                }
            } else { 
                "".to_owned()
            }
        } else {
            "".to_owned()
        }
    }}
}