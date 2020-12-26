// TODO: ASM printing for debug
// TODO: Recursive Pages
// TODO: warn! macro

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CourseOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use x86_64::{
    structures::paging::{MapperAllSizes, Page, PageTable},
    VirtAddr,
};

use CourseOS::{allocator, greeting, memory::{self, BootInfoFrameAllocator}, print, println, shell, task::{keyboard, simple_executor::SimpleExecutor, Task}, wait};
use CourseOS::library::{random, time};
use CourseOS::task::executor::Executor;
use CourseOS::task::stdin;
use CourseOS::vga_buffer::{color, Color, erase};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello, World!\nInitializing...");
    CourseOS::init();

    wait();
    println!("GDT, IDT and PICS are ready. Interrupts enabled.");


    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    wait();
    println!("Virtual memory and heap are ready.");

    #[cfg(test)]
        test_main();

    wait();
    println!("Initialization complete!");
    greeting();

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.spawn(Task::new(shell::lsh()));
    executor.run();
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

async fn async_number() -> u32 {
    42
}


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    CourseOS::hlt_loop();
}

fn color_showcase() {
    color(Color::Black, Color::LightGreen);
    print!("Now colors are reversed!");
    color(Color::Pink, Color::Yellow);
    print!("And now it`s pink and yellow!");
    color(Color::LightGreen, Color::Black);
    println!();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    CourseOS::test_panic_handler(info);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
