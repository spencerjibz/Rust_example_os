#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
//#![no_main]
use core::panic::PanicInfo;
mod serial;
mod vga_buffer;
// panic handler
#[cfg(not(test))]
#[panic_handler]

fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}
// panic handler  in test mode;
#[cfg(test)]
#[panic_handler]

fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!(" Error:{} \n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello again");
    //panic!("some panic method");
    #[cfg(test)]
    test_main();
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    // exit qemu after running all tests
    exit_qemu(QemuExitCode::Success);
}

// trivial assertion
#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion....");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}

/* ------------------------------------------------------------- --------------------------

    HANLDING QEMU EXIT FOR TESTING
-----------------------------------------------------------------------------------------------*/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    //
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(code as u32);
    }
}
