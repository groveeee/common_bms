//定义日期格式化常量

use chrono::{DateTime, Local};

pub const DATE_FORMAT: &str = "%Y-%m-%d";
pub const TIME_FORMAT: &str = "%H:%M:%S";
pub const DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn get_now() -> DateTime<Local> {
    Local::now()
}
pub fn get_current_time_format(now:DateTime<Local>) -> String {
    now.format(DATE_TIME_FORMAT).to_string()
}


pub fn get_current_date_format(now:DateTime<Local>) -> String {
    now.format(DATE_FORMAT).to_string()
}


pub fn get_current_date_time_format(now:DateTime<Local>) -> String {
    now.format(TIME_FORMAT).to_string()
}
