use std::io::{stdout, Write};
use crossterm::{cursor, execute};
use crate::{PADDLE_HEIGHT, TOTAL_HEIGHT, TOTAL_WIDTH};

pub struct Player {
    pub x: u16,
    pub y: i16,
    pub size: i16,
    pub identifier: char,
}

pub struct Players {
    pub paddles: [Player; 2],
}

impl Players {
    pub fn init(paddle_size: i16) -> Self {
        Self {
            paddles: [
                Player { x: 1,  y: (TOTAL_HEIGHT/2) as i16, size: paddle_size, identifier: '▌' },
                Player { x: TOTAL_WIDTH-2, y: (TOTAL_HEIGHT/2) as i16, size: paddle_size, identifier: '▐' },
            ],
        }
    }
    // Dear viewer please slap me in the head after reading this
    pub fn move_paddle(&mut self, index: usize, dy: i16, height: u16) {
        let paddle = &mut self.paddles[index];
        let half_a_paddle = PADDLE_HEIGHT / 2;
        let min_y = 1 + half_a_paddle;
        let max_y = height as i16 - 1 - half_a_paddle;

        let new_y = (paddle.y + dy).clamp(min_y, max_y);
        if new_y == paddle.y {
            return;
        }

        let mut stdout = stdout();


        for i in -half_a_paddle..=half_a_paddle {
            let y = (paddle.y + i) as u16;
            execute!(stdout, cursor::MoveTo(paddle.x, y)).unwrap();
            print!(" ");
        }

        paddle.y = new_y;

        for i in -half_a_paddle..=half_a_paddle {
            let y = (paddle.y + i) as u16;
            execute!(stdout, cursor::MoveTo(paddle.x, y)).unwrap();
            print!("{}", paddle.identifier);
        }

        stdout.flush().unwrap();
    }

    pub fn draw(&self) {
        let mut stdout = stdout();

        for paddle in &self.paddles {
            let half = paddle.size / 2;
            for i in -half..=half {
                let y = (paddle.y + i) as u16;
                execute!(stdout, cursor::MoveTo(paddle.x, y)).unwrap();
                print!("{}", paddle.identifier);
            }
        }
        stdout.flush().unwrap();
    }
}