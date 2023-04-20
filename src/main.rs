#![no_std]
#![no_main]
#![feature(core_intrinsics, lang_items)]

use core::intrinsics;
use core::panic::PanicInfo;

use x86_64::instructions::hlt;

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    intrinsics::abort();
}

#[allow(unused)]
#[derive(Clone, Copy)]
#[repr(u8)]
enum Color {
    Black = 0x0,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    Gray,
    White,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightMagenta,
    Yellow,
    DarkGray,
}

struct Cursor {
    position: isize,
    foreground: Color,
    background: Color,
}

impl Cursor {
    fn color(&self) -> u8 {
        let fg = self.foreground as u8;
        let bg = (self.background as u8) << 4;
        fg | bg
    }

    fn print(&mut self, text: &[u8]) {
        let color = self.color();
        let framebuffer = 0xb8000 as *mut u8;

        for &character in text {
            unsafe {
                framebuffer.offset(self.position).write_volatile(character);
                framebuffer.offset(self.position + 1).write_volatile(color);
            }
            self.position += 2;
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let text = b"Rust in Action";

    let mut cursor = Cursor {
        position: 0,
        foreground: Color::LightCyan,
        background: Color::Black,
    };

    cursor.print(text);

    loop {
        hlt();
    }
}
