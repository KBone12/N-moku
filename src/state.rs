use crossterm::event::{Event, KeyCode, KeyEvent};

use crate::{board::Board, renderer::Renderer, stone::Stone};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Mode {
    VsAi,
    TwoPlayers,
}

impl Mode {
    pub const fn all_modes() -> [Self; 2] {
        [Self::VsAi, Self::TwoPlayers]
    }
}

pub enum State {
    Title {
        n: usize,
        current_mode: Mode,
        to_next: bool,
    },
    Game {
        n: usize,
        mode: Mode,
        board: Board,
        cursor_x: usize,
        cursor_y: usize,
        turn: Stone,
    },
    Finish {
        board: Board,
        winner: Option<Stone>,
    },
}

impl State {
    pub fn new(n: usize) -> Self {
        Self::Title {
            n,
            current_mode: Mode::VsAi,
            to_next: false,
        }
    }

    pub fn render<R: Renderer>(&self, renderer: &mut R) {
        renderer.clear();
        match self {
            Self::Title { current_mode, .. } => {
                renderer.render_title(*current_mode);
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
                if let Some(winner) = winner {
                    renderer.render_winner(*winner);
                } else {
                    renderer.render_drew();
                }
            }
        }
    }

    pub fn process_event(&mut self, event: &Event) {
        match self {
            Self::Title {
                current_mode,
                to_next,
                ..
            } => match event {
                Event::Key(KeyEvent { code, .. }) => match code {
                    KeyCode::Char('k') => {
                        *current_mode = Mode::VsAi;
                    }
                    KeyCode::Char('j') => {
                        *current_mode = Mode::TwoPlayers;
                    }
                    KeyCode::Enter => {
                        *to_next = true;
                    }
                    _ => {}
                },
                _ => {}
            },
            Self::Game {
                n,
                mode,
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

                        if *mode == Mode::VsAi && *turn == Stone::White {
                            crate::ai::action(board, *turn);
                            *turn = if *turn == Stone::Black {
                                Stone::White
                            } else {
                                Stone::Black
                            };
                        }
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
            Self::Title {
                n,
                current_mode,
                to_next,
            } => {
                if *to_next {
                    Some(Self::Game {
                        n: *n,
                        mode: *current_mode,
                        board: Board::new(*n),
                        cursor_x: 0,
                        cursor_y: 0,
                        turn: Stone::Black,
                    })
                } else {
                    None
                }
            }
            Self::Game { board, .. } => board
                .check_winner()
                .map(|winner| Self::Finish {
                    board: board.clone(),
                    winner: Some(winner),
                })
                .or(if board.check_draw() {
                    Some(Self::Finish {
                        board: board.clone(),
                        winner: None,
                    })
                } else {
                    None
                }),
            Self::Finish { .. } => None,
        }
    }
}
