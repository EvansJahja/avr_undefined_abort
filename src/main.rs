#![no_std]
#![no_main]

extern crate core;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[unsafe(no_mangle)]
fn main() -> (){
    panic!("{}", 1);
}
