#![no_std]
#![feature(start)]

use aux_display::{parse, Command};
use embedded_hal::prelude::*;
use gba::{
    bios::soft_reset,
    io::{
        display::{DisplayControlSetting, DisplayMode, DISPCNT},
        sio::{BaudRate, SioSerial},
    },
    vram::bitmap::Mode3,
    Color,
};
use nb::block;
use panic_abort as _;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    DISPCNT.write(
        DisplayControlSetting::new()
            .with_mode(DisplayMode::Mode3)
            .with_bg2(true),
    );

    let mut serial = SioSerial;
    SioSerial::init(BaudRate::Bps115200);

    let mut px: usize = 0;
    let mut py: usize = 0;

    loop {
        match block!(serial.read()) {
            Ok(byte) => {
                if let Some(action) = parse(byte) {
                    match action {
                        Command::ClearDisplay { r, g, b } => {
                            Mode3::dma_clear_to(Color::from_rgb(r, g, b));
                            px = 0;
                            py = 0;
                        }
                        Command::SetPosition { x, y } => {
                            if x < Mode3::WIDTH && y < Mode3::HEIGHT {
                                px = x;
                                py = y;
                            }
                        }
                        Command::WriteData { r, g, b } => {
                            Mode3::write(px, py, Color::from_rgb(r, g, b));
                            px += 1;
                            if px >= Mode3::WIDTH {
                                px = 0;
                                py += 1;
                                if py >= Mode3::HEIGHT {
                                    py = 0;
                                }
                            }
                        }
                    };
                };
            }
            Err(_) => unsafe { soft_reset() },
        };
    }
}
