#![no_std]
#![no_main]

use sanity::{sys};
use sanity::{println, print};

use core::panic::PanicInfo;
use x86_64;

fn hlt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[error] {}", info);
    hlt();
}

fn init() {
    println!("[info] loaded sanity");
    println!("[info] loading idt...");
    sys::idt::init();
    println!("[info] loading gdt...");
    sys::gdt::init();
    println!("[info] loading pic...");
    sys::pic::init();
    println!("[info] sanity init complete");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    println!("no crash :)");
    print!("> ");
    hlt();
}
