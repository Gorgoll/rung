use std::io::{stdout, Write};
use std::time::Duration;
use crossterm::{cursor, execute};
use crate::{TOTAL_HEIGHT, TOTAL_WIDTH};

// https://files.catbox.moe/n8zxkm.gif
pub(crate) async fn handle_ball(mut ball_pos: (u16, u16)) {
    let mut stdout = stdout();
    let mut ball_speed: u8 = 2;
    let mut dx: i8 = 1;
    let mut dy: i8 = 1;
    loop {
        // TODO: Randomize ball speed every bounce
        for _ in 1..ball_speed {
            // Clear ball
            execute!(stdout, cursor::MoveTo(ball_pos.0, ball_pos.1)).unwrap();
            print!(" ");

            // Calculate direction
            ball_pos.0 = (ball_pos.0 as i8 + dx) as u16;
            ball_pos.1 = (ball_pos.1 as i8 + dy) as u16;


            // Draw ball
            execute!(stdout, cursor::MoveTo(ball_pos.0, ball_pos.1)).unwrap();
            print!("O");

            // Check if there are any borders
            if ball_pos.0 <= 1 || ball_pos.0 >= TOTAL_WIDTH - 1 {
                dx *= -1;
            }

            if ball_pos.1 <= 1 || ball_pos.1 >= TOTAL_HEIGHT - 1 {
                dy *= -1;
            }

            stdout.flush().unwrap();
        }
        tokio::time::sleep(Duration::from_millis(40)).await;
    }
}