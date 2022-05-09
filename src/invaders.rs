use std::cmp::max;
use std::time::Duration;

use crate::{
    frame::{Drawable, Frame},
    NUM_COLS, NUM_ROWS,
};
use rusty_time::prelude::Timer;

pub struct Invader {
    pub x: usize,
    pub y: usize,
}

pub struct Invaders {
    pub army: Vec<Invader>,
    timer: Timer,
    direction: i32,
}

const MAX_X: usize = NUM_COLS - 1;
const MIN_X: usize = 0;

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();

        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if (x > 1)
                    && (x < NUM_COLS - 2)
                    && (y > 0)
                    && (y < 9)
                    && (x % 2 == 0)
                    && (y % 2 == 0)
                {
                    army.push(Invader { x, y });
                }
            }
        }

        Self {
            army,
            timer: Timer::from_millis(2000),
            direction: 1, /* Invader starts moving right, so direction initially is +1 */
        }
    }
    pub fn update(&mut self, delta: Duration) -> bool {
        self.timer.update(delta);

        if self.timer.ready {
            self.timer.reset();
            let mut downwards = false;

            if self.direction == -1 {
                let min_x = self
                    .army
                    .iter()
                    .map(|invader| invader.x)
                    .min()
                    .unwrap_or(MIN_X);
                if min_x == MIN_X {
                    self.direction = 1; /* move right */
                    downwards = true;
                }
            } else {
                let max_x = self
                    .army
                    .iter()
                    .map(|invader| invader.x)
                    .max()
                    .unwrap_or(MAX_X);
                if max_x == MAX_X {
                    self.direction = -1; /* move left */
                    downwards = true;
                }
            }

            if downwards {
                let new_duration = max(self.timer.duration.as_millis() - 250, 250);
                self.timer = Timer::from_millis(new_duration as u64);

                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }
            return true;
        }

        false
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            frame[invader.x][invader.y] =
                if (self.timer.time_left.as_secs_f32() / self.timer.duration.as_secs_f32()) > 0.5 {
                    "x"
                } else {
                    "+"
                };
        }
    }
}
