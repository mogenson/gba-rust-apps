#![no_std]
#![feature(start)]

use embedded_hal::prelude::*;
use gba::io::sio::{BaudRate, SioSerial};
use nb::block;
use panic_abort as _;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let mut serial = SioSerial;
    SioSerial::init(BaudRate::Bps115200);

    loop {
        if let Ok(c) = block!(serial.read()) {
            block!(serial.write(c)).ok();
        }
    }
}
