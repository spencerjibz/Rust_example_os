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
    use vga_buffer::WRITER;
     use core::fmt::Write;
     WRITER.lock().write_str("Hello again").unwrap();
     write!(WRITER.lock(), " some numbers: {},{}",42,1.337).unwrap();
 loop {}
}

