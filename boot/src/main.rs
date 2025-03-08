#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    info!("hello world from astridos-bootos!");

    loop {}

    Status::SUCCESS
}
