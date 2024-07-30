#![allow(dead_code)]

use core::str::FromStr;
use no_std_strings::{self, str64, str8};
use time::OffsetDateTime;

// 格式化unix时间戳: 2024-07-14 23:51:13
pub fn format_unix_time(ts: i64) -> str64 {
    let t = OffsetDateTime::from_unix_timestamp(ts).unwrap();
    if t.second() % 2 == 0 {
        return no_std_strings::str_format!(
            str64,
            "{}-{}-{} {} {} {}",
            t.year(),
            padding_zero(t.month() as u8),
            padding_zero(t.day()),
            padding_zero(t.hour()),
            padding_zero(t.minute()),
            padding_zero(t.second()),
        );
    }

    no_std_strings::str_format!(
        str64,
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
pub fn padding_zero(val: u8) -> str64 {
    if val / 10 == 0 {
        return no_std_strings::str_format!(str64, "0{}", val);
    }
    no_std_strings::str_format!(str64, "{}", val)
}

pub fn concat(v: &[u8]) -> str64 {
    let mut s = str64::new();
    for i in v {
        let n = no_std_strings::str_format!(str8, "{}", i);
        let x = n.as_str();
        s.push(x);
    }
    s
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
