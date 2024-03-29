#![allow(dead_code)]
use core::fmt::{self, Result, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> Self {
        Self((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
use volatile::Volatile;
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_WIDTH],
}

// Writer
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_btye(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }
    // method to move every character one line up (top line gets deleted) and start at the beginning of the last line again.
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                // read each character on each row
                let character = self.buffer.chars[row][col].read();

                self.buffer.chars[row - 1][col].write(character); // remove new character to top row
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;

        /*TODO */
    }
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ', //
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
    // read strings
}

impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or new line
                0x20..=0x7e | b'\n' => self.write_btye(byte),
                _ => self.write_btye(0xfe),
            }
        }
    }
}
// Global Writer interface
use lazy_static::lazy_static;
use spin::Mutex;
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// implement write and writeln! macro;

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> Result {
        self.write_string(s);
        Ok(())
    }
}

/* --------------------------------------------------------------------------------------

               Print and Println Macros
----------------------------------------------------------------------------

*/

#[macro_export]
macro_rules! print {
 ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
 }

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

/* --------------------------------------------------------------------------------------
    Testing the VGA BUFFER
--------  --------------------------------    ------------------------------*/
#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

// ensure no panic occurs when multiple lines are printed

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}
// test to verify if printed lines appear on the screen

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line.";
    use x86_64::instructions::interrupts;
    use core::fmt::Write;
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();

        writeln!(&mut writer,"\n{}",s).expect("writeln failed");

        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}

