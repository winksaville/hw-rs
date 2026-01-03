#![no_std]
#![no_main]

use hw_no_std_no_main as _;
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
