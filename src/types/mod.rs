pub mod cond;
pub mod pg;
pub mod rd;
pub mod req;

/// 查询条件
pub use cond::Cond;

/// 值
pub type Val = pg::Val;

/// 数据库连接池
pub type Db = pg::Pool;

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