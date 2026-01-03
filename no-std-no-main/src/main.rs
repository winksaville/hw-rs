#![no_std]
#![no_main]

use syscalls::{Sysno, syscall3};

const STDOUT: usize = 1;

#[unsafe(no_mangle)]
#[unsafe(naked)]
pub extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        "call {rust_main}",
        "mov rdi, rax",  // pass rust_main's return value as exit code
        "call {exit}",
        "ud2",           // safety net if exit somehow returns
        rust_main = sym rust_main,
        exit = sym exit,
    );
}

fn rust_main() -> i32 {
    let s = "Hello, world!\n";

    unsafe {
        syscall3(Sysno::write, STDOUT, s.as_ptr() as usize, s.len()).ok();
    };
    0
}

fn exit(code: usize) -> ! {
    unsafe {
        syscalls::syscall1(Sysno::exit, code).ok();
    }

    // Loop so compiler knows we never return
    #[allow(clippy::empty_loop)]
    loop {}
}

#[cfg(not(test))] // Avoid rust-analyzer error
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    exit(1)
}

// Required to compile in debug/dev mode although never executed
#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}
