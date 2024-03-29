#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(clippy::all)]
extern crate alloc;

use blog_os::{memory::{BootInfoFrameAllocator,self}, println,allocator};

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
entry_point!(kernel_main);
//#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    blog_os::init(); // new_lin

    use x86_64:: VirtAddr;

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // new:initialize a mapper
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
         BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // map an unused page
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );


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
