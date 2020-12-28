#![no_std]
#![feature(start)]

use game_of_life_mode5::Universe;
use gba::{
    io::display::{DisplayControlSetting, DisplayMode, DISPCNT},
    vram::bitmap::{Mode5, Page},
};
use panic_abort as _;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    DISPCNT.write(
        DisplayControlSetting::new()
            .with_mode(DisplayMode::Mode5)
            .with_bg2(true),
    );

    let mut universe = Universe {
        page: Page::Zero,
        width: Mode5::WIDTH as i32,
        height: Mode5::HEIGHT as i32,
    };

    universe.populate();

    loop {
        universe.step();
    }
}
