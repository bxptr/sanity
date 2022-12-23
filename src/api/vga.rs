use core::fmt;

extern crate volatile;
use self::volatile::Volatile;

extern crate lazy_static;
use self::lazy_static::lazy_static;

extern crate spin;
use self::spin::Mutex;

const HEIGHT: usize = 25;
const WIDTH: usize = 80;

#[allow(dead_code)]
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

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column: 0,
        color_entry: ColorEntry::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorEntry(u8);

impl ColorEntry {
    fn new(foreground: Color, background: Color) -> ColorEntry {
        ColorEntry((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct Char {
    ascii: u8,
    color_entry: ColorEntry,
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<Char>; WIDTH]; HEIGHT],
}

pub struct Writer {
    column: usize,
    color_entry: ColorEntry,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.newline(),
            byte => {
                if self.column >= WIDTH {
                    self.newline();
                }
                let row = HEIGHT - 1;
                let column = self.column;
                let color_entry = self.color_entry;
                self.buffer.chars[row][column].write(Char {
                    ascii: byte,
                    color_entry
                });
                self.column += 1;
            }
        }
    }

    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn newline(&mut self) {
        for row in 1..HEIGHT {
            for column in 0..WIDTH {
                let character = self.buffer.chars[row][column].read();
                self.buffer.chars[row - 1][column].write(character);
            }
        }
        self.clear_row(HEIGHT - 1);
        self.column = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = Char {
            ascii: b' ',
            color_entry: self.color_entry,
        };
        for column in 0..WIDTH {
            self.buffer.chars[row][column].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::api::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}