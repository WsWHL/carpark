#![allow(dead_code)]

use core::str::FromStr;
use time::OffsetDateTime;

use crate::format;

use super::strings::Strfmt;

// 格式化unix时间戳: 2024-07-14 23:51:13
pub fn format_time(t: OffsetDateTime) -> Strfmt {
    if t.second() % 2 == 0 {
        return format!(
            "{}-{}-{} {} {} {}",
            t.year(),
            padding_zero(t.month() as u8),
            padding_zero(t.day()),
            padding_zero(t.hour()),
            padding_zero(t.minute()),
            padding_zero(t.second()),
        );
    }

    format!(
        "{}-{}-{} {}:{}:{}",
        t.year(),
        padding_zero(t.month() as u8),
        padding_zero(t.day()),
        padding_zero(t.hour()),
        padding_zero(t.minute()),
        padding_zero(t.second()),
    )
}

// 单数数字左边补0为双数
pub fn padding_zero(val: u8) -> Strfmt {
    if val / 10 == 0 {
        return format!("0{}", val);
    }
    format!("{}", val)
}

// 转换角度
pub fn as_duty(angle: f64) -> u16 {
    ((angle * 29.6) + 1333.3) as u16
}

// 字符串转int
pub fn parse_int<T>(s: &str) -> Result<T, T::Err>
where
    T: FromStr,
{
    s.parse::<T>()
}
