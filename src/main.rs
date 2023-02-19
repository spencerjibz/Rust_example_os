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

    blog_os::init(); // new_lin

    // accessing the page tables;
     use x86_64::registers::control::Cr3;
      let (level_4_page_table,_) = Cr3::read();
     println!("level_4_page_table at: {:?}", level_4_page_table.start_address());
    /* trigger a page fault;

    let ptr = 0x2049b6 as *mut u32;
    // read  from a code page
       unsafe {
         let _x = *ptr;
       }
       println!("read worked");
       // write to a code page
      unsafe{
        *ptr = 42;
      }
       println!("write worked");
       */
    #[allow(unconditional_recursion,dead_code)]
    fn stack_overflow() {
        stack_overflow()
    }
   // stack_overflow();
    #[cfg(test)]
    test_main();
    println!("it didnt crash!");

     blog_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
     blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}
