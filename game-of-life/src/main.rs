#![no_std]
#![feature(start)]

use game_of_life::{Universe, ALIVE, GREEN};
use gba::{
    io::display::{DisplayControlSetting, DisplayMode, DISPCNT},
    palram::index_palram_bg_8bpp,
    vram::bitmap::{Mode4, Page},
};
use panic_abort as _;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    DISPCNT.write(
        DisplayControlSetting::new()
            .with_mode(DisplayMode::Mode4)
            .with_bg2(true),
    );

    index_palram_bg_8bpp(ALIVE).write(GREEN);

    let mut universe = Universe {
        page: Page::Zero,
        width: Mode4::WIDTH as i32,
        height: Mode4::HEIGHT as i32,
    };

    universe.populate();

    loop {
        universe.step();
    }
}
