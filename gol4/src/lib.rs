#![no_std]

use gba::{
    io::display::DISPCNT,
    vram::bitmap::{Mode4, Page},
    Color,
};
use rand::{
    rngs::SmallRng,
    {Rng, SeedableRng},
};

pub const DEAD: u8 = 0;
pub const ALIVE: u8 = 1;
pub const GREEN: Color = Color::from_rgb(0, 31, 0);

pub struct Universe {
    pub page: Page,
    pub width: i32,
    pub height: i32,
}

impl Universe {
    pub fn populate(&self) {
        let mut rng = SmallRng::seed_from_u64(11181981);
        for _ in 0..(self.width * self.height / 8) {
            let x = rng.gen_range(0..self.width) as usize;
            let y = rng.gen_range(0..self.height) as usize;
            Mode4::write(self.page, x, y, ALIVE);
        }
    }

    fn alive(&self, x: i32, y: i32) -> u8 {
        if let Some(cell) = Mode4::read(self.page, x as usize, y as usize) {
            cell
        } else {
            let x = ((x + self.width) % self.width) as usize;
            let y = ((y + self.height) % self.height) as usize;
            Mode4::read(self.page, x, y).unwrap()
        }
    }

    fn next(&self, x: i32, y: i32) -> u8 {
        const NEIGHBORS: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (0, -1),
            (1, 1),
            (1, -1),
            (1, 0),
        ];

        let neighbors: u8 = NEIGHBORS
            .iter()
            .map(|(i, j)| self.alive(x + i, y + j))
            .sum();

        match (self.alive(x, y), neighbors) {
            // rule 1: live cell with less than two live neighbors dies
            (ALIVE, x) if x < 2 => DEAD,
            // rule 2: live cell with 2 or 3 live neighbors lives
            (ALIVE, 2) | (ALIVE, 3) => ALIVE,
            // rule 3: live cell with more than 3 live neighbors dies
            (ALIVE, x) if x > 3 => DEAD,
            // rule 4: dead cell with 3 live neighbors lives
            (DEAD, 3) => ALIVE,
            // no change
            (cell, _) => cell,
        }
    }

    pub fn step(&mut self) {
        let (page, frame) = if self.page == Page::Zero {
            (Page::One, true)
        } else {
            (Page::Zero, false)
        };

        for x in 0..self.width {
            for y in 0..self.height {
                Mode4::write(page, x as usize, y as usize, self.next(x, y));
            }
        }

        DISPCNT.write(DISPCNT.read().with_frame1(frame));
        self.page = page;
    }
}
