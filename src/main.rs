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
    sys::idt::init_idt();
    println!("[info] loading gdt...");
    sys::gdt::init_gdt();
    println!("[info] loading pic...");
    unsafe { sys::idt::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    println!("[info] sanity init complete");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    println!("no crash :)");
    print!("> ");
    hlt();
}
