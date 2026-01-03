#![no_std]

use syscalls::Sysno;

// Declare that main exists in the binary
unsafe extern "Rust" {
    fn main() -> i32;
}

#[unsafe(no_mangle)]
#[unsafe(naked)]
pub extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        "call {main}",
        "mov rdi, rax",  // pass main's return value as exit code
        "call {exit}",
        "ud2",           // safety net if exit somehow returns
        main = sym main,
        exit = sym exit,
    );
}

pub fn exit(code: usize) -> ! {
    unsafe {
        syscalls::syscall1(Sysno::exit, code).ok();
    }

    #[allow(clippy::empty_loop)]
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    exit(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}
