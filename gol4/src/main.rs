#![no_std]
#![feature(start)]

use game_of_life_mode4::{Universe, ALIVE, GREEN};
use gba::{
    io::{
        display::{DisplayControlSetting, DisplayMode, DISPCNT},
        keypad::KEYINPUT,
        timers::{TimerControlSetting, TM0CNT_H, TM0CNT_L},
    },
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

    // start free-running timer
    TM0CNT_H.write(TimerControlSetting::new().with_enabled(true));

    loop {
        // any button pressed
        if KEYINPUT.read() < 0x03FF {
            let seed = TM0CNT_L.read() as u64; // current timer count
            universe.populate(seed); // repopulate universe
        }
        universe.step();
    }
}
