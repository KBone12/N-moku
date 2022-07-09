use std::io::Write;

use crossterm::{
    cursor::{MoveTo, MoveToColumn, MoveToNextLine},
    event::{Event, KeyCode, KeyEvent},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

#[derive(Clone, Copy, Eq, PartialEq)]
enum Stone {
    Empty,
    Black,
    White,
}

struct Board {
    stones: Vec<Vec<Stone>>,
}

impl Board {
    pub fn new(n: usize) -> Self {
        Self {
            stones: vec![vec![Stone::Empty; n]; n],
        }
    }

    pub const fn stones(&self) -> &Vec<Vec<Stone>> {
        &self.stones
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.stones[y][x] == Stone::Empty
    }

    pub fn put(&mut self, stone: Stone, x: usize, y: usize) {
        self.stones[y][x] = stone;
    }
}

struct Renderer<W: Write> {
    writer: W,
    n: usize,
    offset_x: u16,
    offset_y: u16,
}

impl<W: Write> Renderer<W> {
    pub fn new(writer: W, n: usize) -> Self {
        let (width, height) =
            crossterm::terminal::size().expect("Can't get the size of the screen");
        let offset_x = ((width as usize - n) / 2) as _;
        let offset_y = ((height as usize - n) / 2) as _;

        Self {
            writer,
            n,
            offset_x,
            offset_y,
        }
    }

    pub fn render_board(&mut self, board: &Board) {
        crossterm::queue!(self.writer, MoveTo(self.offset_x, self.offset_y))
            .expect("Can't enqueue a command for moving the cursors");
        board.stones().iter().for_each(|row| {
            write!(
                self.writer,
                "{}",
                row.iter()
                    .map(|stone| match stone {
                        Stone::Empty => '-',
                        Stone::Black => 'o',
                        Stone::White => 'x',
                    })
                    .collect::<String>()
            )
            .expect("Can't enqueue commands for drawing a board");
            crossterm::queue!(self.writer, MoveToNextLine(1), MoveToColumn(self.offset_x),)
                .expect("Can't enqueue commands for moving cursors");
        });
        crossterm::queue!(self.writer, MoveTo(self.offset_x, self.offset_y))
            .expect("Can't enqueue a command for moving the cursor");
        self.writer.flush().expect("Can't flush commands");
    }

    pub fn render_cursor(&mut self, x: usize, y: usize) {
        crossterm::execute!(
            self.writer,
            MoveTo(self.offset_x + x as u16, self.offset_y + y as u16)
        )
        .expect("Can't move the cursor");
    }

    pub fn process_event(&mut self, event: &Event) {
        match event {
            Event::Resize(width, height) => {
                self.offset_x = ((*width as usize - self.n) / 2) as _;
                self.offset_y = ((*height as usize - self.n) / 2) as _;
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    crossterm::execute!(std::io::stdout(), EnterAlternateScreen)?;
    let raw_mode_default = crossterm::terminal::is_raw_mode_enabled()?;
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), Clear(ClearType::All))?;

    let n = 3;
    let mut board = Board::new(n);
    let mut renderer = Renderer::new(std::io::stdout(), n);
    let mut black_turn = true;
    let mut cursor_x = 0;
    let mut cursor_y = 0;
    loop {
        renderer.render_board(&board);
        renderer.render_cursor(cursor_x, cursor_y);

        let event = crossterm::event::read()?;
        renderer.process_event(&event);
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => break,
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Char(' ') if board.is_empty(cursor_x, cursor_y) => {
                    board.put(
                        if black_turn {
                            Stone::Black
                        } else {
                            Stone::White
                        },
                        cursor_x,
                        cursor_y,
                    );
                    black_turn = !black_turn;
                }
                KeyCode::Char('B') => {
                    cursor_x = 0;
                    cursor_y = n - 1;
                }
                KeyCode::Char('H') => {
                    cursor_x = 0;
                }
                KeyCode::Char('J') => {
                    cursor_y = n - 1;
                }
                KeyCode::Char('K') => {
                    cursor_y = 0;
                }
                KeyCode::Char('L') => {
                    cursor_x = n - 1;
                }
                KeyCode::Char('N') => {
                    cursor_x = n - 1;
                    cursor_y = n - 1;
                }
                KeyCode::Char('U') => {
                    cursor_x = n - 1;
                    cursor_y = 0;
                }
                KeyCode::Char('Y') => {
                    cursor_x = 0;
                    cursor_y = 0;
                }
                KeyCode::Char('b') => {
                    if cursor_x > 0 {
                        cursor_x -= 1;
                    }
                    if cursor_y < n - 1 {
                        cursor_y += 1;
                    }
                }
                KeyCode::Char('h') => {
                    if cursor_x > 0 {
                        cursor_x -= 1;
                    }
                }
                KeyCode::Char('j') => {
                    if cursor_y < n - 1 {
                        cursor_y += 1;
                    }
                }
                KeyCode::Char('k') => {
                    if cursor_y > 0 {
                        cursor_y -= 1;
                    }
                }
                KeyCode::Char('l') => {
                    if cursor_x < n - 1 {
                        cursor_x += 1;
                    }
                }
                KeyCode::Char('n') => {
                    if cursor_x < n - 1 {
                        cursor_x += 1;
                    }
                    if cursor_y < n - 1 {
                        cursor_y += 1;
                    }
                }
                KeyCode::Char('u') => {
                    if cursor_x < n - 1 {
                        cursor_x += 1;
                    }
                    if cursor_y > 0 {
                        cursor_y -= 1;
                    }
                }
                KeyCode::Char('y') => {
                    if cursor_x > 0 {
                        cursor_x -= 1;
                    }
                    if cursor_y > 0 {
                        cursor_y -= 1;
                    }
                }
                _ => {}
            },
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
