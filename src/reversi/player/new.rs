use super::Player;
use crate::reversi::board::Board;

impl Player {
    pub fn new(side: char, board: [[char; 8]; 8]) -> Self {
        let mut new_board: Board = Board::new();
        new_board.update(board);

        Self {
            side: side,
            board: new_board
        }
    }
}
