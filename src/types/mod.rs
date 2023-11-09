pub mod cond;
pub mod pg;
pub mod rd;
pub mod req;
pub mod token;

/// 查询条件
pub use cond::Cond;

/// 值
pub type Val = pg::Val;

/// 状态：启用
pub const STATUS_ENABLE: Val = Val::I16(crate::consts::STATUS_DISABLE);
/// 状态：禁用
pub const STATUS_DISABLE: Val = Val::I16(crate::consts::STATUS_DISABLE);


/// 数据库连接池
pub type Db = pg::Pool;
pub type Conn = pg::Conn;

/// 数据库连接池配置
pub type PoolOptions = pg::PoolOptions;

/// redis client
pub type Rd = rd::Client;

/// redis cluster client
pub type ClusterRd = rd::ClusterClient;

pub mod pager;
pub mod pagination;
pub mod search;

pub use pagination::Pagination;
pub use search::Search;
pub use pager::Pager;