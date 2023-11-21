/// 页码
pub const PAGE_SIZE: i32 = 15;
/// 默认页码
pub const PAGE_DEFAULT: i32 = 1;

/// 平台系统
pub const PLATFORM_SYSTEM: &'static str = "platform";
/// 平台站点 - header 信息里的key
pub const PLATFORM_KEY: &'static str = "site";

/// 每个平台请求token - header 信息里的 key
pub const AUTHORIZATION: &'static str = "authorization";
/// 默认返回数据类型 key - header
pub const CONTENT_TYPE: &'static str = "content-type";
/// 默认返回数据类型 - json
pub const CONTENT_TYPE_JSON: &'static str = "application/json; charset=utf-8";

/// 资源类型：默认ID
pub const RESOURCE_KIND_DEFAULT: i32 = 1001;
/// 资源类型：系统起始ID
pub const RESOURCE_KIND_BEGIN: i32 = 1006;
/// 资源类型：每个用户最多允许添加分类数量
pub const RESOURCE_KIND_MAX_PER_USER: i64 = 128;

/// 状态：启用
pub const STATUS_ENABLE: i16 = 1;
/// 状态：禁用
pub const STATUS_DISABLE: i16 = 0;

pub mod pg;
pub mod rd;

pub use pg::*;
pub use rd::*;
