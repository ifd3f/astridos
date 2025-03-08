#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;

#[cfg(not(test))]
mod panic_handler;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    info!("hello world from astridos-bootos!");

    loop {}

    Status::SUCCESS
}
