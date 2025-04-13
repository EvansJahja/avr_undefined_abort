#![no_std]
#![no_main]

extern crate core;
extern crate alloc;

use core::{alloc::GlobalAlloc, panic::PanicInfo, str::FromStr};
use alloc::string::String;


struct Alloc;

unsafe impl GlobalAlloc for Alloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        todo!()
    }
}

#[global_allocator]
static GLOBAL: Alloc = Alloc;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}


pub static mut STATMUT : Option<String> = None;

#[unsafe(no_mangle)]
fn main() -> (){
    unsafe {
        STATMUT = Some(String::from_str("hello").unwrap());
    }
    loop {}
}
