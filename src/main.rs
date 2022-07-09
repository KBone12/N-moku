use crossterm::{
    event::{Event, KeyCode, KeyEvent},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

mod board;
mod renderer;
mod state;
mod stone;
use crate::{
    renderer::{CrossTermRenderer, Renderer},
    state::{State, TitleState},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    crossterm::execute!(std::io::stdout(), EnterAlternateScreen)?;
    let raw_mode_default = crossterm::terminal::is_raw_mode_enabled()?;
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), Clear(ClearType::All))?;

    let n = 3;
    let mut renderer = CrossTermRenderer::new(std::io::stdout(), n);
    let mut state: Box<dyn State<_, Event = _>> = Box::new(TitleState::new()) as _;
    loop {
        state.render(&mut renderer);

        let event = crossterm::event::read()?;
        renderer.process_event(&event);
        state.process_event(&event);
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => break,
            _ => {}
        }

        if let Some(next) = state.next_state() {
            state = next;
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
