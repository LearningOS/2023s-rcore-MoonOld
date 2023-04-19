//! The panic handler

use crate::sbi::shutdown;
use core::panic::PanicInfo;

// panic_handler是一个特殊的函数，当panic!宏被调用时，会调用panic_handler函数
// 使用这个宏可以把下面的函数声明为panic_handler
#[panic_handler]
/// panic handler
fn panic(info: &PanicInfo) -> ! {
    // 如果info有location信息，就打印文件名，行号，panic信息
    if let Some(location) = info.location() {
        println!(
            "[kernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        // 没有的话就只打印panic信息
        println!("[kernel] Panicked: {}", info.message().unwrap());
    }
    shutdown()
}
