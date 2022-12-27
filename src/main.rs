#![no_std]
#![no_main]

use sanity::{sys};
use sanity::{println};

use core::panic::PanicInfo;
use x86_64;
use bootloader::{BootInfo, entry_point};

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

fn init(boot_info: &'static BootInfo) {
    println!("[info] loaded sanity");
    println!("[info] loading idt...");
    sys::idt::init();
    println!("[info] loading gdt...");
    sys::gdt::init();
    println!("[info] loading pic...");
    sys::pic::init();
    println!("[info] loading mem...");
    sys::mem::init(boot_info);
    println!("[info] sanity init complete");
}

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);
    let mut executor = sys::executor::Executor::new();
    executor.spawn(sys::executor::Task::new(sys::keyboard::print_keypresses()));
    executor.run();
}

entry_point!(kernel_main);