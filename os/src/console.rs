//! SBI console driver, for text output
use crate::sbi::console_putchar;
use core::fmt::{self, Write};

// 这个结构体实现了Write trait，所以可以使用write_fmt方法，而且没有成员变量，所以可以直接实例化
struct Stdout;

// 为Stdout实现Write trait
impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

// 虽然没有实现write_fmt方法，但是因为实现了Write trait，所以可以直接使用
// write_fmt方法会将fmt::Arguments转换为字符串，然后调用write_str方法
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// Print! to the host console using the format string and arguments.
#[macro_export]
macro_rules! print {
    
    ($fmt: literal $(, $($arg: tt)+)?) => {
        // fmt::Arguments可以用宏初始化，比如format_args!宏，用法是format_args!("{} {}", 1, 2)
        $crate::console::print(format_args!($fmt $(, $($arg)+)?))
    }
}

/// Println! to the host console using the format string and arguments.
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?))
    }
}
