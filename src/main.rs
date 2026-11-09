#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(runix::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use runix::{hlt_loop, println};

entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
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

    //
    //

    #[cfg(test)]
    test_main();

    hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    runix::test_panic_handler(info)
}

// tests
