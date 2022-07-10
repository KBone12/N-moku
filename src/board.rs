use crate::stone::Stone;

#[derive(Clone)]
pub struct Board {
    n: usize,
    stones: Vec<Vec<Option<Stone>>>,
}

impl Board {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            stones: vec![vec![None; n]; n],
        }
    }

    pub const fn stones(&self) -> &Vec<Vec<Option<Stone>>> {
        &self.stones
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.stones[y][x].is_none()
    }

    pub fn check_winner(&self) -> Option<Stone> {
        // Check horizontal
        for y in 0..self.n {
            if self.stones[y]
                .iter()
                .all(|stone| *stone == Some(Stone::Black))
            {
                return Some(Stone::Black);
            } else if self.stones[y]
                .iter()
                .all(|stone| *stone == Some(Stone::White))
            {
                return Some(Stone::White);
            }
        }

        // Check vertical
        for x in 0..self.n {
            if self.stones.iter().all(|row| row[x] == Some(Stone::Black)) {
                return Some(Stone::Black);
            } else if self.stones.iter().all(|row| row[x] == Some(Stone::White)) {
                return Some(Stone::White);
            }
        }

        // Check diagonal
        let mut result = self.stones[0][0];
        for k in 0..self.n {
            if self.stones[k][k] != result {
                result = None;
                break;
            }
        }
        if result.is_some() {
            return result;
        }
        result = self.stones[0][self.n - 1];
        for k in 0..self.n {
            if self.stones[k][self.n - 1 - k] != result {
                result = None;
                break;
            }
        }
        if result.is_some() {
            return result;
        }

        None
    }

    pub fn put(&mut self, stone: Option<Stone>, x: usize, y: usize) {
        self.stones[y][x] = stone;
    }
}
