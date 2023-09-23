use crate::{gdt, hlt_loop};
use crate::{print, println};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]

pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}
impl From<InterruptIndex> for u8 {
    fn from(index: InterruptIndex) -> Self {
        index as u8
    }
}
impl From<InterruptIndex> for usize {
    fn from(index: InterruptIndex) -> Self {
        index as Self
    }
}
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
         idt[InterruptIndex::Timer.into()].set_handler_fn(timer_interrupt_handler);
         //  add keyboard interrupt
          idt[InterruptIndex::Keyboard.into()].set_handler_fn(keyboard_interrupt_handler);
          // add page interrupt handler;
          idt.page_fault.set_handler_fn(page_fault_handler);
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

// new Double Handler;
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.into());
    }
}
/*-----------------------------------------------------------------------
            KEYBOARD INTERRUPT
* ----------------------------------------------------------------

*/

// keyboard interrupt handler0x
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // inteprete the scancodes from PS/2
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
            Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
        );
    }

    // println!("{}", scancode);
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe {
        port.read()
    };
  
  if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {

     if let Some(key) = keyboard.process_keyevent(key_event){
          match key {
             DecodedKey::Unicode(character) => println!("{character}"),
            DecodedKey::RawKey(key) => println!("{key:?}"),
            
            }
     }

    }
 
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.into());
    }
}

/*-------------------------------------------------------------------

 PAGE FAULT INTERRUPT
------------------------------------------------------------------- */
use x86_64::structures::idt::PageFaultErrorCode;
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;
    println!("EXCEPTION: PAGEFAULT");
    println!("Accessed address:{:?}", Cr2::read());
    println!("Error code:{error_code:?}");
    println!("{stack_frame:#?}");

    hlt_loop();
}
