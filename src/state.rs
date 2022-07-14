use crossterm::event::{Event, KeyCode, KeyEvent};

use crate::{board::Board, renderer::Renderer, stone::Stone};

pub enum State {
    Title {
        n: usize,
        to_next: bool,
    },
    Game {
        n: usize,
        board: Board,
        cursor_x: usize,
        cursor_y: usize,
        turn: Stone,
    },
    Finish {
        board: Board,
        winner: Stone,
    },
}

impl State {
    pub fn new(n: usize) -> Self {
        Self::Title { n, to_next: false }
    }

    pub fn render<R: Renderer>(&self, renderer: &mut R) {
        renderer.clear();
        match self {
            Self::Title { .. } => {
                renderer.render_title();
            }
            Self::Game {
                board,
                cursor_x,
                cursor_y,
                ..
            } => {
                renderer.render_board(&board);
                renderer.render_cursor(*cursor_x, *cursor_y);
            }
            Self::Finish { board, winner } => {
                renderer.render_board(&board);
                renderer.render_winner(*winner);
            }
        }
    }

    pub fn process_event(&mut self, event: &Event) {
        match self {
            Self::Title { to_next, .. } => match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char(' '),
                    ..
                }) => {
                    *to_next = true;
                }
                _ => {}
            },
            Self::Game {
                n,
                board,
                cursor_x,
                cursor_y,
                turn,
            } => match event {
                Event::Key(KeyEvent { code, .. }) => match code {
                    KeyCode::Char(' ') if board.is_empty(*cursor_x, *cursor_y) => {
                        board.put(Some(*turn), *cursor_x, *cursor_y);
                        *turn = if *turn == Stone::Black {
                            Stone::White
                        } else {
                            Stone::Black
                        };
                    }
                    KeyCode::Char('B') => {
                        *cursor_x = 0;
                        *cursor_y = *n - 1;
                    }
                    KeyCode::Char('H') => {
                        *cursor_x = 0;
                    }
                    KeyCode::Char('J') => {
                        *cursor_y = *n - 1;
                    }
                    KeyCode::Char('K') => {
                        *cursor_y = 0;
                    }
                    KeyCode::Char('L') => {
                        *cursor_x = *n - 1;
                    }
                    KeyCode::Char('N') => {
                        *cursor_x = *n - 1;
                        *cursor_y = *n - 1;
                    }
                    KeyCode::Char('U') => {
                        *cursor_x = *n - 1;
                        *cursor_y = 0;
                    }
                    KeyCode::Char('Y') => {
                        *cursor_x = 0;
                        *cursor_y = 0;
                    }
                    KeyCode::Char('b') => {
                        if *cursor_x > 0 {
                            *cursor_x -= 1;
                        }
                        if *cursor_y < *n - 1 {
                            *cursor_y += 1;
                        }
                    }
                    KeyCode::Char('h') => {
                        if *cursor_x > 0 {
                            *cursor_x -= 1;
                        }
                    }
                    KeyCode::Char('j') => {
                        if *cursor_y < *n - 1 {
                            *cursor_y += 1;
                        }
                    }
                    KeyCode::Char('k') => {
                        if *cursor_y > 0 {
                            *cursor_y -= 1;
                        }
                    }
                    KeyCode::Char('l') => {
                        if *cursor_x < *n - 1 {
                            *cursor_x += 1;
                        }
                    }
                    KeyCode::Char('n') => {
                        if *cursor_x < *n - 1 {
                            *cursor_x += 1;
                        }
                        if *cursor_y < *n - 1 {
                            *cursor_y += 1;
                        }
                    }
                    KeyCode::Char('u') => {
                        if *cursor_x < *n - 1 {
                            *cursor_x += 1;
                        }
                        if *cursor_y > 0 {
                            *cursor_y -= 1;
                        }
                    }
                    KeyCode::Char('y') => {
                        if *cursor_x > 0 {
                            *cursor_x -= 1;
                        }
                        if *cursor_y > 0 {
                            *cursor_y -= 1;
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            Self::Finish { .. } => {}
        }
    }

    pub fn next_state(&self) -> Option<Self> {
        match self {
            Self::Title { n, to_next } => {
                if *to_next {
                    Some(Self::Game {
                        n: *n,
                        board: Board::new(*n),
                        cursor_x: 0,
                        cursor_y: 0,
                        turn: Stone::Black,
                    })
                } else {
                    None
                }
            }
            Self::Game { board, .. } => board.check_winner().map(|winner| Self::Finish {
                board: board.clone(),
                winner,
            }),
            Self::Finish { .. } => None,
        }
    }
}
