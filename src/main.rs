#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(clippy::all)]

use blog_os::{memory, println};

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

entry_point!(kernel_main);
//#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    blog_os::init(); // new_lin

    use x86_64::{structures::paging::Translate, VirtAddr};

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // new:initialize a mapper
    let mapper = unsafe { memory::init(phys_mem_offset) };

    let addresses = [
        // the identity-map vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page,
        0x0100_0020_1a10_u64,
        // virtual address mapped to physical address 0,
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }
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
    #[allow(unconditional_recursion, dead_code)]
    fn stack_overflow() {
        stack_overflow()
    }
    // stack_overflow();
    #[cfg(test)]
    test_main();
    println!("it didn't crash!");

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
