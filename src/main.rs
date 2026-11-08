#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(runix::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use runix::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    //test
    let vga_buff = 0xb8000 as *mut u8;

    for i in 0..25 * 80 {
        unsafe {
            *vga_buff.offset(i as isize * 2 + 1) = 0x44;
        }
    }
    //test

    println!("hello world");

    runix::init();

    #[cfg(test)]
    test_main();

    println!("i am alive");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    runix::test_panic_handler(info)
}

// tests
