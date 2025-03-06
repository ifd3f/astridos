#![no_std]
#![no_main]

use serial::init_serial;

#[cfg(not(test))]
mod panic_handler;

mod serial;

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

#[no_mangle]
pub fn kmain() {
    const HELLO: &[u8] = b"Hello World from AstridOS!";

    /*
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *VGA_BUFFER.offset(i as isize * 2) = byte;
            *VGA_BUFFER.offset(i as isize * 2 + 1) = 0xb;
        }
    } */

    unsafe {
        init_serial();
        serial::puts(HELLO);
    }

    loop {}
}
