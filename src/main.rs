#![no_std]
#![no_main]

use sanity::{sys};
use sanity::{println};

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[error] {}", info);
    loop {}
}

fn init() {
    println!("[info] loaded sanity");
    println!("[info] loading idt...");
    sys::idt::init_idt();
    println!("[info] loading gdt...");
    sys::gdt::init_gdt();
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };
    println!("[info] sanity init complete");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    println!("no crash :)");
    loop {}
}
