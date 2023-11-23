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
/// 资源：每个用户最多允许添加资源数量
pub const RESOURCE_MAX_PER_USER: i64 = 1024;

// 10: 待审核 20: 处理中 30: 已完成 40: 已取消 50: 挂起(表示任务完成一部分钱不够了) 60: 已拒绝
pub const TASK_STATUS_DEFAULT: i16 = 10;
pub const TASK_STATUS_PROCESSING: i16 = 20;
pub const TASK_STATUS_COMPLETED: i16 = 30;
pub const TASK_STATUS_CANCELED: i16 = 40;
pub const TASK_STATUS_SUSPENDED: i16 = 50;
pub const TASK_STATUS_REJECTED: i16 = 60;
// 10: 免担保 20: 担保任务
pub const TASK_ENSURE_TYPE_DEFAULT: i16 = 10;
pub const TASK_ENSURE_TYPE_GUARANTEE: i16 = 20;
// 0: 个人任务 1: 团队任务
pub const TASK_TEAM_TYPE_NO: i16 = 0;
pub const TASK_TEAM_TYPE_YES: i16 = 1;
// 0: 固定佣金 1: 比例佣金
pub const TASK_COMMISSION_TYPE_FIXED: i16 = 0;
pub const TASK_COMMISSION_TYPE_RATIO: i16 = 1;
// 0: 有时间限制 1: 无时间限制
pub const TASK_TIME_LIMIT_NO: i16 = 0;
pub const TASK_TIME_LIMIT_YES: i16 = 1;
// 0: 不允许同IP领取 1: 允许同IP领取
pub const TASK_IP_LIMIT_NO: i16 = 0;
pub const TASK_IP_LIMIT_YES: i16 = 1;
// 0: 不允许直接领取 1: 允许直接领取
pub const TASK_DIRECT_LIMIT_NO: i16 = 0;
pub const TASK_DIRECT_LIMIT_YES: i16 = 1;

/// 状态：启用
pub const STATUS_ENABLE: i16 = 1;
/// 状态：禁用
pub const STATUS_DISABLE: i16 = 0;

pub mod pg;
pub mod rd;

pub use pg::*;
pub use rd::*;
