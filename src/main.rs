#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CourseOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use CourseOS::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, World{}", "!");
    CourseOS::init();

    #[cfg(test)]
    test_main();

    CourseOS::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    CourseOS::hlt_loop();
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

