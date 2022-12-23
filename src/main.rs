#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub mod api;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn init() {
    println!("[info] loaded sanity");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    println!("no crash :)");
    loop {}
}
