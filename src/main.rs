mod ball_handler;
mod spawn_border;
mod player;

use crossterm::{cursor, execute, terminal};
use crossterm::event::{poll, read, Event, KeyCode};
use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use crate::ball_handler::{Ball};
use crate::player::Players;
use crate::spawn_border::draw_border;

pub static TOTAL_WIDTH: u16 = 100;
pub static TOTAL_HEIGHT: u16 = 40;
pub static PADDLE_HEIGHT: i16 = 10;

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    execute!(stdout(), cursor::Hide)?;

    while run_game()? {}

    terminal::disable_raw_mode()?;
    execute!(stdout(), cursor::Show)?;
    Ok(())
}

fn run_game() -> std::io::Result<bool> {
    draw_border(TOTAL_WIDTH, TOTAL_HEIGHT);

    let mut players = Players::init(PADDLE_HEIGHT);
    players.draw();

    let mut ball = Ball::init(TOTAL_WIDTH / 2, TOTAL_HEIGHT / 2);

    let tick = Duration::from_millis(40);
    let mut last_tick = Instant::now();

    loop {
        // THIS SUCKS
        while poll(Duration::from_millis(0))? {
            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Char('w') => players.move_paddle(0, -1, TOTAL_HEIGHT),
                    KeyCode::Char('s') => players.move_paddle(0,  1, TOTAL_HEIGHT),
                    KeyCode::Up        => players.move_paddle(1, -1, TOTAL_HEIGHT),
                    KeyCode::Down      => players.move_paddle(1,  1, TOTAL_HEIGHT),
                    KeyCode::Esc       => return Ok(false),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick {
            if let Some(player) = ball.update(TOTAL_WIDTH, TOTAL_HEIGHT, &players) {
                show_game_over(player, TOTAL_WIDTH, TOTAL_HEIGHT);
                loop {
                    if let Event::Key(key) = read()? {
                        match key.code {
                            KeyCode::Char('r') => return Ok(true),
                            KeyCode::Esc       => return Ok(false),
                            _ => {}
                        }
                    }
                }
            }

            ball.draw();
            last_tick = Instant::now();
        }
    }
}

fn show_game_over(player: u8, width: u16, height: u16) {
    let mut stdout = stdout();
    // No ternary op sucks
    let winner = if player == 0 { "Player 1 (left)" } else { "Player 2 (right)" };

    let cx = width / 2;
    let cy = height / 2;

    execute!(stdout, cursor::MoveTo(cx.saturating_sub(winner.len() as u16 / 2), cy - 1)).unwrap();
    print!("{} scores ", winner);

    stdout.flush().unwrap();
}