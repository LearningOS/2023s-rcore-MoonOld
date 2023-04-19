//ref:: https://github.com/andre-richter/qemu-exit
use core::arch::asm;

const EXIT_SUCCESS: u32 = 0x5555; // Equals `exit(0)`. qemu successful exit

const EXIT_FAILURE_FLAG: u32 = 0x3333;
const EXIT_FAILURE: u32 = exit_code_encode(1); // Equals `exit(1)`. qemu failed exit
const EXIT_RESET: u32 = 0x7777; // qemu reset

// 总体来说这个文件就是定义了一个trait，然后实现了这个trait，这个trait就是用来退出qemu的
// 退出qemu的方式就是通过sbi调用，sbi调用的编号是8，然后将EXIT_SUCCESS作为参数传入
// 这个EXIT_SUCCESS就是一个魔数，qemu会根据这个魔数来判断是正常退出还是异常退出
// 这里没有ecall，触发sbi的方式是
pub trait QEMUExit {
    /// Exit with specified return code.
    ///
    /// Note: For `X86`, code is binary-OR'ed with `0x1` inside QEMU.
    fn exit(&self, code: u32) -> !;

    /// Exit QEMU using `EXIT_SUCCESS`, aka `0`, if possible.
    ///
    /// Note: Not possible for `X86`.
    fn exit_success(&self) -> !;

    /// Exit QEMU using `EXIT_FAILURE`, aka `1`.
    fn exit_failure(&self) -> !;
}

/// RISCV64 configuration
pub struct RISCV64 {
    /// Address of the sifive_test mapped device.
    addr: u64,
}

/// Encode the exit code using EXIT_FAILURE_FLAG.
const fn exit_code_encode(code: u32) -> u32 {
    (code << 16) | EXIT_FAILURE_FLAG
}

impl RISCV64 {
    /// Create an instance.
    pub const fn new(addr: u64) -> Self {
        RISCV64 { addr }
    }
}

impl QEMUExit for RISCV64 {
    /// Exit qemu with specified exit code.
    fn exit(&self, code: u32) -> ! {
        // If code is not a special value, we need to encode it with EXIT_FAILURE_FLAG.
        let code_new = match code {
            EXIT_SUCCESS | EXIT_FAILURE | EXIT_RESET => code,
            _ => exit_code_encode(code),
        };

        unsafe {
            // 这段汇编的作用是将code_new的值写入到addr指向的内存中
            // addr是退出的设备，在这句汇编之后，qemu就会退出，实际上是
            asm!(
                "sw {0}, 0({1})",
                in(reg)code_new, in(reg)self.addr
            );

            // 这里调用panic!()是不可行的，因为这里有可能是panic!()处理程序中的最后一个表达式，所以可以解决无限loop的问题
            // For the case that the QEMU exit attempt did not work, transition into an infinite
            // loop. Calling `panic!()` here is unfeasible, since there is a good chance
            // this function here is the last expression in the `panic!()` handler
            // itself. This prevents a possible infinite loop.
            // wfi指令是等待中断指令，wfi是wait for interrupt的缩写
            // 这里在等待的中断就是qemu退出的中断
            loop {
                asm!("wfi", options(nomem, nostack));
            }
        }
    }

    fn exit_success(&self) -> ! {
        self.exit(EXIT_SUCCESS);
    }

    fn exit_failure(&self) -> ! {
        self.exit(EXIT_FAILURE);
    }
}

// 这个地址是qemu中的一个设备，这个设备的作用是退出qemu
const VIRT_TEST: u64 = 0x100000;

// 这个HANDLE是一个全局变量，是一个RISCV64类型的变量，用来退出qemu
pub const QEMU_EXIT_HANDLE: RISCV64 = RISCV64::new(VIRT_TEST);
