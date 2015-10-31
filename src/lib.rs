use std::fmt;
use std::io::{self, stdout, Write};

#[doc(hidden)]
pub fn _try_print(args: fmt::Arguments) -> io::Result<()> {
    stdout().write_fmt(args)
}

/// Macro for printing to the standard output.
///
/// Equivalent to the `print!` macro except it does not panic if it fails to
/// write to stdout.
///
/// Note that stdout is frequently line-buffered by default so it may be
/// necessary to use `io::stdout().flush()` to ensure the output is emitted
/// immediately.
///
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate try_print;
/// # fn main() {
/// use std::io::{self, Write};
///
/// try_print!("this ").unwrap();
/// try_print!("will ").unwrap();
/// try_print!("be ").unwrap();
/// try_print!("on ").unwrap();
/// try_print!("the ").unwrap();
/// try_print!("same ").unwrap();
/// try_print!("line ").unwrap();
///
/// io::stdout().flush().unwrap();
///
/// try_print!("this string has a newline, why not choose println! instead?\n").unwrap();
///
/// io::stdout().flush().unwrap();
/// # }
/// ```
#[macro_export]
macro_rules! try_print {
    ($($arg:tt)*) => ($crate::_try_print(format_args!($($arg)*)));
}

/// Macro for printing to the standard output, with a newline.
///
/// Use the `format!` syntax to write data to the standard output.
/// See `std::fmt` for more information.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate try_print;
/// # fn main() {
/// use try_print;
/// try_println!("hello there!").unwrap();
/// try_println!("format {} arguments", "some").unwrap();
/// # }
/// ```
#[macro_export]
macro_rules! try_println {
    ($fmt:expr) => (try_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (try_print!(concat!($fmt, "\n"), $($arg)*));
}
