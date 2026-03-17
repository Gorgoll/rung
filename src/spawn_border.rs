use std::io::{stdout, Write};
use crossterm::{cursor, execute, terminal};
use crossterm::terminal::ClearType;

// TODO: make this preferably not clear the terminal (tho fullscreen is yet to be decided)
pub(crate) fn draw_border(width: u16, height: u16) {
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
