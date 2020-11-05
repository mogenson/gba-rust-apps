#![no_std]

pub enum Command {
    ClearDisplay { r: u16, g: u16, b: u16 },
    SetPosition { x: usize, y: usize },
    WriteData { r: u16, g: u16, b: u16 },
}

pub fn parse(byte: u8) -> Option<Command> {
    match byte {
        0xFF => (), // clear display
        0xFE => (), // set position
        0xFD => (), // write data
        byte => (),
    }

    None
}

/*
 * 0xFF clear display. read three bytes: r g b
 * 0xFE set position. read 2 bytes: x y
 * 0xFD write data. read bytes for ever: r g b r g b etc
 */
