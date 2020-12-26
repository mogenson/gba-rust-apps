#![no_std]
#![feature(start)]

use gba::io::{
    irq::{set_irq_handler, IrqEnableSetting, IrqFlags, BIOS_IF, IE, IF, IME},
    sio::{BaudRate, IoControlSetting, SioControlSetting, SioMode, RCNT, SIOCNT, SIODATA8},
};
use panic_abort as _;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    RCNT.write(IoControlSetting::new());
    SIOCNT.write(
        SioControlSetting::new()
            .with_baud_rate(BaudRate::Bps115200)
            .with_data_length_8bit(true)
            .with_mode(SioMode::Uart)
            .with_rx_enable(true)
            .with_tx_enable(true)
            .with_irq_enable(true),
    );

    set_irq_handler(irq_handler);
    IE.write(IrqFlags::new().with_serial(true));
    IME.write(IrqEnableSetting::IRQ_YES);

    loop {}
}

extern "C" fn irq_handler(flags: IrqFlags) {
    // clear all interrupts by writing the set bits back
    BIOS_IF.write(BIOS_IF.read());
    IF.write(IF.read());
    if flags.serial() {
        if !SIOCNT.read().rx_empty() {
            let mut data = SIODATA8.read() as u8 as char;
            data.make_ascii_uppercase();
            SIODATA8.write(data as u16);
        }
    }
}
