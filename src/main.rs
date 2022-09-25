#![no_std]
#![no_main]
//#![no_main]
use core::panic::PanicInfo;
mod vga_buffer;
// panic handler
#[panic_handler]

fn panic(info: &PanicInfo) -> ! { 
  println!("{}", info);
    loop{}
}
// panic

#[no_mangle]
pub extern "C" fn _start() -> ! {
   
    println!("Hello again");
     println!( "some numbers: {},{}",42,1.337);
     panic!("some panic message");
 loop {}
}

