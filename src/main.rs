use std::io::Write;

use crossterm::{
    cursor::{Hide, MoveTo, MoveToColumn, MoveToNextLine, Show},
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
    n: usize,
    stones: Vec<Vec<Stone>>,
}

impl Board {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            stones: vec![vec![Stone::Empty; n]; n],
        }
    }

    pub const fn stones(&self) -> &Vec<Vec<Stone>> {
        &self.stones
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.stones[y][x] == Stone::Empty
    }

    pub fn check_winner(&self) -> Stone {
        // Check horizontal
        for y in 0..self.n {
            if self.stones[y].iter().all(|stone| *stone == Stone::Black) {
                return Stone::Black;
            } else if self.stones[y].iter().all(|stone| *stone == Stone::White) {
                return Stone::White;
            }
        }

        // Check vertical
        for x in 0..self.n {
            if self.stones[..][x]
                .iter()
                .all(|stone| *stone == Stone::Black)
            {
                return Stone::Black;
            } else if self.stones[..][x]
                .iter()
                .all(|stone| *stone == Stone::White)
            {
                return Stone::White;
            }
        }

        // Check diagonal
        let mut result = self.stones[0][0];
        for k in 0..self.n {
            if self.stones[k][k] != result {
                result = Stone::Empty;
                break;
            }
        }
        if result != Stone::Empty {
            return result;
        }
        result = self.stones[0][self.n - 1];
        for k in 0..self.n {
            if self.stones[k][self.n - 1 - k] != result {
                result = Stone::Empty;
                break;
            }
        }
        if result != Stone::Empty {
            return result;
        }

        Stone::Empty
    }

    pub fn put(&mut self, stone: Stone, x: usize, y: usize) {
        self.stones[y][x] = stone;
    }
}

struct Renderer<W: Write> {
    writer: W,
    n: usize,
    center_x: u16,
    center_y: u16,
}

impl<W: Write> Renderer<W> {
    pub fn new(writer: W, n: usize) -> Self {
        let (width, height) =
            crossterm::terminal::size().expect("Can't get the size of the screen");
        let center_x = width / 2;
        let center_y = height / 2;

        Self {
            writer,
            n,
            center_x,
            center_y,
        }
    }

    pub fn clear(&mut self) {
        crossterm::execute!(self.writer, Clear(ClearType::All)).expect("Can't clear the screen");
    }

    pub fn render_title(&mut self) {
        crossterm::queue!(
            self.writer,
            Hide,
            MoveTo(self.center_x - 3, self.center_y - 1)
        )
        .expect("Can't enqueue commands for move the cursor");
        write!(self.writer, "N moku").expect("Can't render the title");
        crossterm::queue!(self.writer, MoveTo(self.center_x - 11, self.center_y + 1))
            .expect("Can't enqueue commands for move the cursor");
        write!(self.writer, "Press [SPACE] to start").expect("Can't render the title");
        self.writer.flush().expect("Can't flush commands");
    }

    pub fn render_board(&mut self, board: &Board) {
        crossterm::queue!(
            self.writer,
            MoveTo(
                self.center_x - self.n as u16 / 2,
                self.center_y - self.n as u16 / 2
            )
        )
        .expect("Can't enqueue a command for moving the cursor");
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
            crossterm::queue!(
                self.writer,
                MoveToNextLine(1),
                MoveToColumn(self.center_x - self.n as u16 / 2),
            )
            .expect("Can't enqueue commands for moving the cursor");
        });
        crossterm::queue!(
            self.writer,
            MoveTo(
                self.center_x - self.n as u16 / 2,
                self.center_y - self.n as u16 / 2
            )
        )
        .expect("Can't enqueue a command for moving the cursor");
        self.writer.flush().expect("Can't flush commands");
    }

    pub fn render_cursor(&mut self, x: usize, y: usize) {
        crossterm::execute!(
            self.writer,
            MoveTo(
                self.center_x - self.n as u16 / 2 + x as u16,
                self.center_y - self.n as u16 / 2 + y as u16
            ),
            Show,
        )
        .expect("Can't move the cursor");
    }

    pub fn render_winner(&mut self, winner: Stone) {
        crossterm::queue!(
            self.writer,
            Hide,
            MoveTo(self.center_x - 8, self.center_y - self.n as u16)
        )
        .expect("Can't enqueue commands for move the cursor");
        write!(
            self.writer,
            "{} player WIN!!!",
            if winner == Stone::Black { "1st" } else { "2nd" }
        )
        .expect("Can't render the winner");
        crossterm::queue!(
            self.writer,
            MoveTo(self.center_x - 9, self.center_y + self.n as u16)
        )
        .expect("Can't enqueue commands for move the cursor");
        write!(self.writer, "Press [ESC] to quit").expect("Can't render a text");
        self.writer.flush().expect("Can't flush commands");
    }

    pub fn process_event(&mut self, event: &Event) {
        match event {
            Event::Resize(width, height) => {
                self.center_x = *width / 2;
                self.center_y = *height / 2;
            }
            _ => {}
        }
    }
}

#[derive(Eq, PartialEq)]
enum GameState {
    Title,
    Game,
    Finish,
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
    let mut state = GameState::Title;
    let mut winner = Stone::Empty;
    loop {
        renderer.clear();
        match state {
            GameState::Title => {
                renderer.render_title();
            }
            GameState::Game => {
                renderer.render_board(&board);
                renderer.render_cursor(cursor_x, cursor_y);
            }
            GameState::Finish => {
                renderer.render_board(&board);
                renderer.render_winner(winner);
            }
        }

        let event = crossterm::event::read()?;
        renderer.process_event(&event);
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => break,
            Event::Key(KeyEvent { code, .. }) => match state {
                GameState::Title => match code {
                    KeyCode::Char(' ') => {
                        state = GameState::Game;
                    }
                    _ => {}
                },
                GameState::Game => match code {
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
            },
            _ => {}
        }

        if state == GameState::Game {
            winner = board.check_winner();
            match winner {
                Stone::Black | Stone::White => {
                    state = GameState::Finish;
                }
                _ => {}
            }
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
