#![no_std]
#![no_main]

#![feature(abi_x86_interrupt)]

mod api;
mod sys;
use crate::sys::idt::{init_idt};

use core::panic::PanicInfo;

extern crate x86_64;
use self::x86_64::instructions::interrupts;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[error] {}", info);
    loop {}
}

fn init() {
    println!("[info] loaded sanity");
    println!("[info] loading idt...");
    init_idt();
    x86_64::instructions::interrupts::int3();
    println!("[info] sanity init complete");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    println!("no crash :)");
    loop {}
}
