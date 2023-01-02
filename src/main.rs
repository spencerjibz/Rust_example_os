#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(clippy::all)]

use blog_os::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    blog_os::init(); // new_line
                     // trigger a page fault;
    #[allow(unconditional_recursion,dead_code)]
    fn stack_overflow() {
        stack_overflow()
    }
   // stack_overflow();
    #[cfg(test)]
    test_main();
    println!("it didnt crash!");

    loop {
         use blog_os::print;
         print!("-");
         // slow down the timer;
         for _ in 0..10000 {}
    }
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}
