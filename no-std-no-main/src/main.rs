//! Minimal no_std binary example.

#![no_std]
#![no_main]

// Link the runtime crate which provides `_start`, panic handler, and other
// low-level infrastructure required for this no_std binary.
extern crate hw_no_std_no_main;

use syscalls::{Sysno, syscall3};

const STDOUT: usize = 1;

#[unsafe(no_mangle)]
fn main() -> i32 {
    let s = "Hello, world!\n";

    unsafe {
        syscall3(Sysno::write, STDOUT, s.as_ptr() as usize, s.len()).ok();
    };
    0
}
