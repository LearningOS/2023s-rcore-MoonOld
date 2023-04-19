//! The main module and entrypoint
//!
//! The operating system and app also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality [`clear_bss()`]. (See its source code for
//! details.)
//!
//! We then call [`println!`] to display `Hello, world!`.

// 拒绝没注释的编译
#![deny(missing_docs)]
#![deny(warnings)]
// exclude std的依赖
#![no_std]
#![no_main]
//允许使用PanicInfo::message
#![feature(panic_info_message)]

// 引入log
use core::arch::global_asm;
use log::*;

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

// 引入board并标注其path
#[path = "boards/qemu.rs"]
mod board;

// 引入入口汇编
global_asm!(include_str!("entry.asm"));

/// clear BSS segment
/// bss段是用来存放未初始化或者初始化为0的全局变量的
pub fn clear_bss() {
    extern "C" {
        // 使用linker提供的标记点
        fn sbss();
        fn ebss();
    }
    // sbss转化为usize，然后遍历sbss到ebss的地址空间
    (sbss as usize..ebss as usize).for_each(|a| 
        unsafe {
            // 地址强转为字节指针 覆写0，volatile保证可见性
            (a as *mut u8)
            .write_volatile(0) });
}

/// the rust entry-point of os
#[no_mangle]
pub fn rust_main() -> ! {
    // 这下面这些函数都是linker提供的标记点
    extern "C" {
        fn stext(); // begin addr of text segment
        fn etext(); // end addr of text segment
        fn srodata(); // start addr of Read-Only data segment
        fn erodata(); // end addr of Read-Only data ssegment
        fn sdata(); // start addr of data segment
        fn edata(); // end addr of data segment
        fn sbss(); // start addr of BSS segment
        fn ebss(); // end addr of BSS segment
        fn boot_stack_lower_bound(); // stack lower bound
        fn boot_stack_top(); // stack top
    }
    clear_bss();
    logging::init();
    println!("[kernel] Hello, world!");
    trace!(
        "[kernel] .text [{:#x}, {:#x})",
        stext as usize,
        etext as usize
    );
    debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as usize, edata as usize
    );
    warn!(
        "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top as usize, boot_stack_lower_bound as usize
    );
    error!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);

    use crate::board::QEMUExit;
    // 使用全局变量QEMU_EXIT_HANDLE调用exit_success
    crate::board::QEMU_EXIT_HANDLE.exit_success(); // CI autotest success
                                                   //crate::board::QEMU_EXIT_HANDLE.exit_failure(); // CI autoest failed
}
