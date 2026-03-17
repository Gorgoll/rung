mod ball_handler;
mod spawn_border;

use crossterm::{cursor, execute, terminal};
use std::io::{stdout};
use crate::ball_handler::handle_ball;
use crate::spawn_border::draw_border;

// I will later organize this code
// For now I'm trying to get the game to somewhat usable state
static mut IS_GAME_RUNNING: bool = true;
// TODO: make this somewhat usable with various terminal sizes
// hardcoding numbers not the best idea but works for now
static TOTAL_WIDTH: u16= 70;
static TOTAL_HEIGHT: u16 = 40;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode().unwrap();
    let _ = execute!(stdout(),cursor::Hide);
    draw_border(TOTAL_WIDTH, TOTAL_HEIGHT);

    let ball_pos = (TOTAL_WIDTH / 2, TOTAL_HEIGHT / 2);
    handle_ball(ball_pos).await;
    Ok(())
}