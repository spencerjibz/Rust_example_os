use crate::gdt;
use crate::println;
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

/*  ----------------------------------------------------------------
 *  Interrupt Descriptor Table
 *  ----------------------------------------------------------------
 */
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);


        // load a valid TSS and interrupt stack tables
         unsafe {
              idt.double_fault.set_handler_fn(double_fault_handler)
              .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
         }
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION:BREAKPOINT\n{:#?}", stack_frame)
}

// new Double Handler;
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION:DOUBLE FAULT\n{:#?}", stack_frame)
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}

/* ----------------------------------------------------------------
 *   PIC
 * ----------------------------------------------------------------
 */
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });
