/// 页码
pub const PAGE_SIZE: i32 = 15;
/// 默认页码
pub const PAGE_DEFAULT: i32 = 1;
/// 平台系统
pub const PLATFORM_SYSTEM: &'static str = "platform";

/// 状态：启用
pub const STATUS_ENABLE: i16 = 1;
/// 状态：禁用
pub const STATUS_DISABLE: i16 = 0;

pub mod pg;
pub mod rd;

pub use pg::*;
pub use rd::*;