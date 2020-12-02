#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CourseOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use CourseOS::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop{}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    CourseOS::test_panic_handler(info);
}

#[test_case]
fn test_println() {
    println!("test_println output");
}