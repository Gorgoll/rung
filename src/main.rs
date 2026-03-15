use crossterm::{cursor, execute, terminal};
use std::io::{stdout, Write};
use std::time::Duration;
use crossterm::terminal::ClearType;
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

    draw_border(TOTAL_WIDTH, TOTAL_HEIGHT);

    let ball_pos = (TOTAL_WIDTH / 2, TOTAL_HEIGHT / 2);
    handle_ball(ball_pos).await;
    Ok(())
}

// TODO: make this preferably not clear the terminal (tho fullscreen is yet to be decided)
fn draw_border(width: u16, height: u16) {
    let mut stdout = stdout();

    let _ = execute!(stdout,terminal::Clear(ClearType::All));

    // top and bottom 🫵
    for x in 0..width+1 {
        execute!(stdout, cursor::MoveTo(x, 0)).unwrap();
        print!("#");

        execute!(stdout, cursor::MoveTo(x, height)).unwrap();
        print!("#");
    }
    // left and right
    for y in 0..height {
        println!();
        execute!(stdout, cursor::MoveTo(0, y)).unwrap();
        print!("#");

        execute!(stdout, cursor::MoveTo(width, y)).unwrap();
        print!("#");
    }

    stdout.flush().unwrap();
}

// https://files.catbox.moe/n8zxkm.gif
async fn handle_ball(mut ball_pos: (u16, u16)) {
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

            // Check if there are any borders
            if ball_pos.0 <= 1 || ball_pos.0 >= TOTAL_WIDTH - 1 {
                dx *= -1;
            }

            if ball_pos.1 <= 1 || ball_pos.1>= TOTAL_HEIGHT - 1 {
                dy *= -1;
            }

            // Draw ball
            execute!(stdout, cursor::MoveTo(ball_pos.0, ball_pos.1)).unwrap();
            print!("O");

            stdout.flush().unwrap();
        }
        tokio::time::sleep(Duration::from_millis(40)).await;
    }
}