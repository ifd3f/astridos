#![no_std]
#![no_main]

#[cfg(not(test))]
mod panic_handler;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

fn main() {}
