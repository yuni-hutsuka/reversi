mod new;
mod random;
mod select;

use crate::reversi::board::Board;

pub struct Player {
    side: char,
    board: Board,
}
