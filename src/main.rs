//tells rust not to use the std lib
#![no_std]
//tells rust that we don't want to use the normal entry point
#![no_main]
// we will use a custom test framework as we don't have a std
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::println;
use core::panic::PanicInfo;

//this prevents name mangling, i.e, this function will keep it's name
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World! {}", 12);

    blog_os::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!!");
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
    blog_os::test_panic_handler(info)
}
