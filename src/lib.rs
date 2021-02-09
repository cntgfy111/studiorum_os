#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(type_name_of_val)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![feature(const_in_array_repeat_expressions)]
#![feature(wake_trait)]
#![feature(iterator_fold_self)]

extern crate alloc;

use core::panic::PanicInfo;

#[cfg(test)]
use bootloader::{BootInfo, entry_point};

use vga_buffer::WRITER;

#[cfg(test)]
entry_point!(test_kernel_main);

pub mod allocator;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod vga_buffer;
pub mod task;
pub mod library;
pub mod shell;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn greeting() {
    println!(r#"   _____ _             _ _                             ____   _____
  / ____| |           | (_)                           / __ \ / ____|
 | (___ | |_ _   _  __| |_  ___  _ __ _   _ _ __ ___ | |  | | (___
  \___ \| __| | | |/ _` | |/ _ \| '__| | | | '_ ` _ \| |  | |\___ \
  ____) | |_| |_| | (_| | | (_) | |  | |_| | | | | | | |__| |____) |
 |_____/ \__|\__,_|\__,_|_|\___/|_|   \__,_|_| |_| |_|\____/|_____/"#)
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn wait() {
    for _ in 1..20 {
        x86_64::instructions::hlt();
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

pub trait Testable {
    fn run(&self);
}

impl<T: Fn()> Testable for T {
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("\n\nRunning {} tests...", tests.len());
    for test in tests {
        test.run();
    }
    serial_println!();
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error, {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
}
