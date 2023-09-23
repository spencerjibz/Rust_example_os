#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(clippy::all)]

use blog_os::println;

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

entry_point!(kernel_main);
//#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    blog_os::init(); // new_lin
    use blog_os::memory::active_level_4_table;
    use x86_64::VirtAddr;
    use x86_64::structures::paging::PageTable;

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {i}: {entry:?}");

            
        let phys = entry.frame().unwrap().start_address();
        let virt = phys.as_u64() + boot_info.physical_memory_offset;
        let ptr = VirtAddr::new(virt).as_mut_ptr();
        let l3_table: &PageTable = unsafe { &*ptr };

        for (i, entry) in l3_table.iter().enumerate() {
            if !entry.is_unused() {
                println!("  L3 Entry {}: {:?}", i, entry);
            }
        }
        }



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
