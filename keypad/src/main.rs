#![no_std]
#![feature(start)]

use gba::{
    io::{
        display::{DisplayControlSetting, DisplayMode, DISPCNT},
        irq::{set_irq_handler, IrqEnableSetting, IrqFlags, IE, IF, IME},
        keypad::{read_key_input, KeyInterruptSetting, KEYCNT},
    },
    vram::bitmap::Mode3,
    Color,
};
use panic_abort as _;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    DISPCNT.write(
        DisplayControlSetting::new()
            .with_mode(DisplayMode::Mode3)
            .with_bg2(true),
    );

    Mode3::write(120, 80, Color::from_rgb(31, 0, 0));

    set_irq_handler(irq_handler);
    KEYCNT.write(
        KeyInterruptSetting::new()
            .with_a(true)
            .with_b(true)
            .with_irq_enabled(true),
    );
    IE.write(IrqFlags::new().with_keypad(true));
    IME.write(IrqEnableSetting::IRQ_YES);

    loop {}
}

extern "C" fn irq_handler(flags: IrqFlags) {
    IF.write(IF.read());
    if flags.keypad() {
        let keys = read_key_input();
        if keys.a() {
            Mode3::write(120, 80, Color::from_rgb(0, 31, 0));
        } else if keys.b() {
            Mode3::write(120, 80, Color::from_rgb(0, 0, 31));
        }
    }
}
