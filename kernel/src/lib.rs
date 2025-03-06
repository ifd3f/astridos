#![no_std]
#![no_main]

#[cfg(not(test))]
mod panic_handler;

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

#[no_mangle]
pub fn kmain() {
    const HELLO: &[u8] = b"Hello World!";

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *VGA_BUFFER.offset(i as isize * 2) = byte;
            *VGA_BUFFER.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
