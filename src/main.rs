#![no_std]
#![no_main]

use sanity::{sys};
use sanity::{println, print};

use core::panic::PanicInfo;
use x86_64;
use bootloader::{BootInfo, entry_point};

extern crate alloc;
use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};

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

fn heap_test() {
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));
}

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);
    println!("no crash :)");
    heap_test();
    print!("> ");
    hlt();
}

entry_point!(kernel_main);