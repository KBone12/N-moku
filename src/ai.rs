use crate::{board::Board, stone::Stone};

pub fn action(board: &mut Board, ai_stone: Stone) {
    let actions = available_actions(board);
    if actions.len() == 0 {
        return;
    }

    let i = fastrand::usize(..actions.len());
    let (x, y) = actions[i];
    board.put(Some(ai_stone), x, y);
}

fn available_actions(board: &Board) -> Vec<(usize, usize)> {
    board
        .stones()
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter().enumerate().filter_map(
                move |(x, stone)| {
                    if stone.is_some() {
                        None
                    } else {
                        Some((x, y))
                    }
                },
            )
        })
        .flatten()
        .collect()
}
