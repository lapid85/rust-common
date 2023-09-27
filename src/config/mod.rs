use dotenv;

/// 获得配置内容
pub mod env;

// 加载配置文件
pub fn load_env() {
    dotenv::dotenv().ok();
}
