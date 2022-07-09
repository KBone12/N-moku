use crossterm::event::{KeyCode, KeyEvent};

use crate::{board::Board, renderer::Renderer, stone::Stone};

pub trait State<R: Renderer> {
    type Event;

    fn render(&self, renderer: &mut R);
    fn process_event(&mut self, event: &Self::Event);
    fn next_state(&self) -> Option<Box<dyn State<R, Event = Self::Event>>>;
}

pub struct TitleState {
    to_next_state: bool,
}

impl TitleState {
    pub fn new() -> Self {
        Self {
            to_next_state: false,
        }
    }
}

impl<R: Renderer> State<R> for TitleState {
    type Event = crossterm::event::Event;

    fn render(&self, renderer: &mut R) {
        renderer.clear();
        renderer.render_title();
    }

    fn process_event(&mut self, event: &Self::Event) {
        match event {
            Self::Event::Key(KeyEvent {
                code: KeyCode::Char(' '),
                ..
            }) => {
                self.to_next_state = true;
            }
            _ => {}
        }
    }

    fn next_state(&self) -> Option<Box<dyn State<R, Event = Self::Event>>> {
        if self.to_next_state {
            Some(Box::new(GameState::new()))
        } else {
            None
        }
    }
}

pub struct GameState {
    n: usize,
    board: Board,
    cursor_x: usize,
    cursor_y: usize,
    turn: Stone,
}

impl GameState {
    pub fn new() -> Self {
        let n = 3;
        Self {
            n,
            board: Board::new(n),
            cursor_x: 0,
            cursor_y: 0,
            turn: Stone::Black,
        }
    }
}

impl<R: Renderer> State<R> for GameState {
    type Event = crossterm::event::Event;

    fn render(&self, renderer: &mut R) {
        renderer.clear();
        renderer.render_board(&self.board);
        renderer.render_cursor(self.cursor_x, self.cursor_y);
    }

    fn process_event(&mut self, event: &Self::Event) {
        match event {
            Self::Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Char(' ') if self.board.is_empty(self.cursor_x, self.cursor_y) => {
                    self.board
                        .put(Some(self.turn), self.cursor_x, self.cursor_y);
                    self.turn = if self.turn == Stone::Black {
                        Stone::White
                    } else {
                        Stone::Black
                    };
                }
                KeyCode::Char('B') => {
                    self.cursor_x = 0;
                    self.cursor_y = self.n - 1;
                }
                KeyCode::Char('H') => {
                    self.cursor_x = 0;
                }
                KeyCode::Char('J') => {
                    self.cursor_y = self.n - 1;
                }
                KeyCode::Char('K') => {
                    self.cursor_y = 0;
                }
                KeyCode::Char('L') => {
                    self.cursor_x = self.n - 1;
                }
                KeyCode::Char('N') => {
                    self.cursor_x = self.n - 1;
                    self.cursor_y = self.n - 1;
                }
                KeyCode::Char('U') => {
                    self.cursor_x = self.n - 1;
                    self.cursor_y = 0;
                }
                KeyCode::Char('Y') => {
                    self.cursor_x = 0;
                    self.cursor_y = 0;
                }
                KeyCode::Char('b') => {
                    if self.cursor_x > 0 {
                        self.cursor_x -= 1;
                    }
                    if self.cursor_y < self.n - 1 {
                        self.cursor_y += 1;
                    }
                }
                KeyCode::Char('h') => {
                    if self.cursor_x > 0 {
                        self.cursor_x -= 1;
                    }
                }
                KeyCode::Char('j') => {
                    if self.cursor_y < self.n - 1 {
                        self.cursor_y += 1;
                    }
                }
                KeyCode::Char('k') => {
                    if self.cursor_y > 0 {
                        self.cursor_y -= 1;
                    }
                }
                KeyCode::Char('l') => {
                    if self.cursor_x < self.n - 1 {
                        self.cursor_x += 1;
                    }
                }
                KeyCode::Char('n') => {
                    if self.cursor_x < self.n - 1 {
                        self.cursor_x += 1;
                    }
                    if self.cursor_y < self.n - 1 {
                        self.cursor_y += 1;
                    }
                }
                KeyCode::Char('u') => {
                    if self.cursor_x < self.n - 1 {
                        self.cursor_x += 1;
                    }
                    if self.cursor_y > 0 {
                        self.cursor_y -= 1;
                    }
                }
                KeyCode::Char('y') => {
                    if self.cursor_x > 0 {
                        self.cursor_x -= 1;
                    }
                    if self.cursor_y > 0 {
                        self.cursor_y -= 1;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn next_state(&self) -> Option<Box<dyn State<R, Event = Self::Event>>> {
        self.board
            .check_winner()
            .map(|winner| Box::new(FinishState::new(self.board.clone(), winner)) as _)
    }
}

pub struct FinishState {
    board: Board,
    winner: Stone,
}

impl FinishState {
    pub fn new(board: Board, winner: Stone) -> Self {
        Self { board, winner }
    }
}

impl<R: Renderer> State<R> for FinishState {
    type Event = crossterm::event::Event;

    fn render(&self, renderer: &mut R) {
        renderer.clear();
        renderer.render_board(&self.board);
        renderer.render_winner(self.winner);
    }

    fn process_event(&mut self, _event: &Self::Event) {}

    fn next_state(&self) -> Option<Box<dyn State<R, Event = Self::Event>>> {
        None
    }
}
