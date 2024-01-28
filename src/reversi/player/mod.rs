mod new;
mod random;

use crate::reversi::board::Board;

pub struct Player {
    side: char,
    board: Board,
}
