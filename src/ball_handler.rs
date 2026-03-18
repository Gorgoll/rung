use std::io::{stdout, Write};
use crossterm::{cursor, execute};
use rand::{RngExt};

use crate::player::Players;

pub struct Ball {
    pos: (u16, u16),
    dx: i8,
    dy: i8,
    speed: u8,
}

impl Ball {
    pub fn init(x: u16, y: u16) -> Self {
        Self {
            pos: (x, y),
            dx: 1,
            dy: 1,
            speed: 2,
        }
    }

    pub fn update(&mut self, width: u16, height: u16, players: &Players) -> Option<u8> {
        let mut stdout = stdout();
        let steps = self.speed;

        for _ in 0..steps {
            // Erase current position
            execute!(stdout, cursor::MoveTo(self.pos.0, self.pos.1)).unwrap();
            print!(" ");
            stdout.flush().unwrap();

            // Game over
            if self.pos.0 <= 1 {
                return Some(1); // player 2 scores
            }
            if self.pos.0 >= width - 1 {
                return Some(0); // player 1 scores
            }

            let mut bounced = false;

            // Wall bounce
            if self.pos.1 <= 1 || self.pos.1 >= height - 1 {
                self.dy *= -1;
                self.speed = random_speed();
                bounced = true;
            }

            // Paddle collisions
            // Works I guess...
            // Tho sometimes it's a bit broken but im not dealing with that
            for paddle in &players.paddles {
                let half = paddle.size / 2;
                let paddle_top    = (paddle.y - half) as u16;
                let paddle_bottom = (paddle.y + half) as u16;

                let hitting_left  = self.dx < 0 && self.pos.0 == paddle.x + 1;
                let hitting_right = self.dx > 0 && self.pos.0 == paddle.x - 1;

                if (hitting_left || hitting_right)
                    && self.pos.1 >= paddle_top
                    && self.pos.1 <= paddle_bottom
                {
                    self.dx *= -1;
                    self.speed = random_speed();
                    bounced = true;

                    let hit_offset = self.pos.1 as i16 - paddle.y;
                    if hit_offset < 0 {
                        self.dy = -1;
                    } else if hit_offset > 0 {
                        self.dy = 1;
                    }
                }
            }

            // Advance pos
            self.pos.0 = (self.pos.0 as i32 + self.dx as i32)
                .clamp(1, (width - 1) as i32) as u16;
            self.pos.1 = (self.pos.1 as i32 + self.dy as i32)
                .clamp(1, (height - 1) as i32) as u16;

            if bounced {
                break;
            }
        }

        stdout.flush().unwrap();
        None
    }

    pub fn draw(&self) {
        let mut stdout = stdout();
        execute!(stdout, cursor::MoveTo(self.pos.0, self.pos.1)).unwrap();
        print!("O");
        stdout.flush().unwrap();
    }
}

fn random_speed() -> u8 {
    rand::rng().random_range(1..=2)
}