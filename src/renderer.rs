use std::io::Write;

use crossterm::{
    cursor::{Hide, MoveTo, MoveToColumn, MoveToNextLine, Show},
    terminal::{Clear, ClearType},
};

use crate::{board::Board, stone::Stone};

pub trait Renderer {
    type Event;

    fn clear(&mut self);
    fn render_title(&mut self);
    fn render_board(&mut self, board: &Board);
    fn render_cursor(&mut self, x: usize, y: usize);
    fn render_winner(&mut self, winner: Stone);

    fn process_event(&mut self, event: &Self::Event);
}

pub struct CrossTermRenderer<W: Write> {
    writer: W,
    n: usize,
    center_x: u16,
    center_y: u16,
}

impl<W: Write> CrossTermRenderer<W> {
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
}

impl<W: Write> Renderer for CrossTermRenderer<W> {
    type Event = crossterm::event::Event;

    fn clear(&mut self) {
        crossterm::execute!(self.writer, Clear(ClearType::All)).expect("Can't clear the screen");
    }

    fn render_title(&mut self) {
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

    fn render_board(&mut self, board: &Board) {
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
                        Some(Stone::Black) => 'o',
                        Some(Stone::White) => 'x',
                        None => '-',
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

        self.writer.flush().expect("Can't flush commands");
    }

    fn render_cursor(&mut self, x: usize, y: usize) {
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

    fn render_winner(&mut self, winner: Stone) {
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

    fn process_event(&mut self, event: &Self::Event) {
        match event {
            Self::Event::Resize(width, height) => {
                self.center_x = *width / 2;
                self.center_y = *height / 2;
            }
            _ => {}
        }
    }
}
