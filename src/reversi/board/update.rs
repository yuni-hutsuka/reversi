use super::Board;

impl Board {
    pub fn update(&mut self, new_board: [[char; 8]; 8]) {
        for i in 0..8 {
            for j in 0..8 {
                self.board[i][j] = new_board[i][j];
            }
        }
    }
}
