use chrono::{DateTime, Datelike, Duration, Local, NaiveDateTime, Timelike};

/// 获取当前时间
#[inline]
pub fn now() -> DateTime<Local> {
    Local::now()
}

pub fn now_utc_micro() -> i64 {
    let current_time = chrono::Utc::now();
    let timestamp = current_time.timestamp_millis();
    timestamp
}

/// 获取今天凌晨的时间戳
pub fn today_begin() -> DateTime<Local> {
    let current_time = now();
    let hour = current_time.with_hour(0).unwrap();
    let minute = hour.with_minute(0).unwrap();
    let begin = minute.with_second(0).unwrap();
    begin
}

/// 获取今天结束的时间戳
pub fn today_end() -> DateTime<Local> {
    let current_time = now();
    let hour = current_time.with_hour(23).unwrap();
    let minute = hour.with_minute(59).unwrap();
    let second = minute.with_second(59).unwrap();
    second
}

/// 一周开始时间
pub fn week_begin(date: &DateTime<Local>) -> DateTime<Local> {
    let weekday = date.weekday();
    let current_time = now();
    let days_from_monday = weekday.num_days_from_monday();
    let start_of_week = current_time - Duration::days(days_from_monday as i64);
    let hour = start_of_week.with_hour(0).unwrap();
    let minute = hour.with_minute(0).unwrap();
    let begin = minute.with_second(0).unwrap();
    begin
}

/// 一周结束时间
pub fn week_end(date: &DateTime<Local>) -> DateTime<Local> {
    let weekday = date.weekday();
    let current_time = now();
    let days_from_monday = weekday.num_days_from_monday();
    let end_of_week = current_time + Duration::days(6 - days_from_monday as i64);
    let hour = end_of_week.with_hour(23).unwrap();
    let minute = hour.with_minute(59).unwrap();
    let second = minute.with_second(59).unwrap();
    second
}

/// 一月开始时间
pub fn month_start() -> DateTime<Local> {
    let current_time = now();
    let day = current_time.with_day(1).unwrap();
    let hour = day.with_hour(0).unwrap();
    let minute = hour.with_minute(0).unwrap();
    let begin = minute.with_second(0).unwrap();
    begin
}

/// 获取月末时间
pub fn month_end() -> DateTime<Local> {
    let current_time = now();
    let year = current_time.year();
    let month = current_time.month();
    let days_in_month = chrono::NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or_else(|| chrono::NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap())
        .pred_opt()
        .unwrap()
        .day();
    let day = current_time.with_day(days_in_month).unwrap();
    let hour = day.with_hour(23).unwrap();
    let minute = hour.with_minute(59).unwrap();
    let second = minute.with_second(59).unwrap();
    second
}

/// 获取当前日期 - 元组
pub fn date_tuple() -> (i32, u32, u32) {
    let current_time: DateTime<Local> = now();
    let year = current_time.year();
    let month = current_time.month();
    let day = current_time.day();
    (year, month, day)
}

/// 获取当前日期时间 - 元组
pub fn datetime_tuple() -> (i32, u32, u32, u32, u32, u32) {
    let current_time: DateTime<Local> = now();
    let year = current_time.year();
    let month = current_time.month();
    let day = current_time.day();
    let hour = current_time.hour();
    let minute = current_time.minute();
    let second = current_time.second();
    (year, month, day, hour, minute, second)
}

/// 得到当前时间戳, 单位: 秒
#[inline]
pub fn timestamp() -> i64 {
    let local: DateTime<Local> = now();
    let timestamp = local.timestamp();
    timestamp
}

/// 得到当前时间 - 字符串
#[inline]
pub fn datetime() -> String {
    let local: DateTime<Local> = now();
    let datetime_str = local.format("%Y-%m-%d %H:%M:%S").to_string();
    datetime_str
}

/// 将日期字符串转换为时间戳 - 非utc
pub fn str_to_timestamp(datetime_str: &str) -> i64 {
    let format_str = "%Y-%m-%d %H:%M:%S";
    match NaiveDateTime::parse_from_str(datetime_str, format_str) {
        Ok(v) => v.timestamp(),
        Err(err) => {
            println!("解析时间失败: {:?}", err);
            return 0;
        }
    }
}

/// 将时间戳转换为日期字符串 - 非utc
pub fn timestamp_to_str(timestamp: i64) -> String {
    let Some(datetime) = NaiveDateTime::from_timestamp_opt(timestamp, 0) else {
        return "2001-01-01 00:00:00".to_owned();
    };
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// 默认开始时间 - 本日开始
pub fn default_start() -> DateTime<Local> {
    let current = now();
    let hour = current.with_hour(0).unwrap();
    let minute = hour.with_minute(0).unwrap();
    let second = minute.with_second(0).unwrap();
    second
}

/// 默认结束时间 - 本日结束
pub fn default_end() -> DateTime<Local> {
    let current = now();
    let hour = current.with_hour(23).unwrap();
    let minute = hour.with_minute(59).unwrap();
    let second = minute.with_second(59).unwrap();
    second
}
