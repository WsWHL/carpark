#![allow(dead_code)]
use core::{
    fmt::{self, Display, Write},
    str::FromStr,
};

use heapless::String;

#[doc = "字符串格式化"]
pub struct Strfmt<const N: usize = 64> {
    buf: String<N>,
}

impl<const N: usize> Strfmt<N> {
    pub fn new() -> Self {
        Self {
            buf: String::<N>::new(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        Self {
            buf: String::<N>::from_str(s).unwrap(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut buf = String::<N>::new();
        for i in bytes {
            write!(&mut buf, "{}", i).unwrap();
        }

        Self { buf }
    }

    pub fn replace_all(&mut self, from: &str, to: &str) {
        let mut result = String::<N>::new();
        let mut last_index = 0;

        for (index, _) in self.buf.match_indices(from) {
            result.push_str(&self.buf[last_index..index]).unwrap();
            result.push_str(to).unwrap();
            last_index = index + from.len();
        }

        result.push_str(&self.buf[last_index..]).unwrap();
        self.buf = result;
    }

    pub fn as_str(&self) -> &str {
        self.buf.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
}

impl<const N: usize> Write for Strfmt<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.buf.write_str(s)
    }
}

impl<const N: usize> Display for Strfmt<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str()).unwrap();
        Ok(())
    }
}

pub fn new_strfmt(args: fmt::Arguments) -> Strfmt {
    let mut w = Strfmt::new();
    fmt::write(&mut w, args).unwrap();
    w
}

#[doc = "字符串格式化"]
#[macro_export]
macro_rules! format {
    ($($args:tt)*) => {
        $crate::strings::new_strfmt(format_args!($($args)*))
    };
}
