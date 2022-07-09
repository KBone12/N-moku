use std::io::Write;

use crossterm::{
    cursor::{MoveTo, MoveToColumn, MoveToNextLine},
    event::{Event, KeyCode, KeyEvent},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

#[derive(Clone, Copy)]
enum Stone {
    Empty,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    crossterm::execute!(std::io::stdout(), EnterAlternateScreen)?;
    let raw_mode_default = crossterm::terminal::is_raw_mode_enabled()?;
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), Clear(ClearType::All))?;

    loop {
        let (screen_width, screen_height) = crossterm::terminal::size()?;
        let screen_width = screen_width as usize;
        let screen_height = screen_height as usize;
        let n = 3;
        let offset_x = (screen_width - n) / 2;
        let offset_y = (screen_height - n) / 2;
        let board = vec![vec![Stone::Empty; n]; n];
        crossterm::queue!(std::io::stdout(), MoveTo(offset_x as _, offset_y as _))?;
        board
            .iter()
            .map(|row| {
                write!(
                    std::io::stdout(),
                    "{}",
                    row.iter()
                        .map(|stone| match stone {
                            Stone::Empty => '-',
                        })
                        .collect::<String>()
                )?;
                crossterm::queue!(
                    std::io::stdout(),
                    MoveToNextLine(1),
                    MoveToColumn(offset_x as _),
                )
            })
            .collect::<Result<_, _>>()?;
        crossterm::queue!(std::io::stdout(), MoveTo(offset_x as _, offset_y as _))?;
        std::io::stdout().flush()?;

        match crossterm::event::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => break,
            _ => {}
        }
    }

    if raw_mode_default {
        crossterm::terminal::enable_raw_mode()?;
    } else {
        crossterm::terminal::disable_raw_mode()?;
    }
    crossterm::execute!(std::io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
