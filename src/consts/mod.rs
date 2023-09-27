/// 页码
pub const PAGE_SIZE: i32 = 15;
/// 默认页码
pub const PAGE_DEFAULT: i32 = 1;

pub mod pg;
pub mod rd;

pub use pg::*;
pub use rd::*;