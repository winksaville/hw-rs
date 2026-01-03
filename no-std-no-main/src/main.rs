#![no_std]
#![no_main]

use syscalls::{Sysno, syscall3};

const STDOUT: usize = 1;

#[unsafe(no_mangle)]
#[unsafe(naked)]
pub extern "C" fn _start() -> ! {
    // Kernel enters with RSP = 16n
    core::arch::naked_asm!(
        "call {rust_main}",
        "ud2", // if rust_main returns we execute an undefined instruction
               // https://linuxvox.com/blog/what-s-the-purpose-of-the-ud2-opcode-in-the-linux-kernel/
        rust_main = sym rust_main,
    );
}

fn rust_main() -> ! {
    let s = "Hello, world!\n";

    unsafe {
        syscall3(Sysno::write, STDOUT, s.as_ptr() as usize, s.len()).ok();
    };
    exit(0);
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
