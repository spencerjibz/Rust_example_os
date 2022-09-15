#![no_std]
#![no_main]
//#![no_main]
use core::panic::PanicInfo;
mod vga_buffer;
// panic handler
#[panic_handler]

fn panic(_info: &PanicInfo) -> ! { 
    loop{}
}
// panic

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
 loop {}
}

