use core::fmt::{self, Display, Write};

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

    pub fn as_str(&self) -> &str {
        self.buf.as_str()
    }

    pub fn add_slice(&mut self, bytes: &[u8]) {
        for i in bytes {
            write!(self, "{}", i).unwrap();
        }
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
