use super::Board;

impl Board {
    pub fn new() -> Self {
        Self {
            board: [
                ['e', 'e', 'e', 'e', 'e', 'e', 'e', 'e'],
                ['e', 'e', 'e', 'e', 'e', 'e', 'e', 'e'],
                ['e', 'e', 'e', 'e', 'e', 'e', 'e', 'e'],
                ['e', 'e', 'e', 'w', 'b', 'e', 'e', 'e'],
                ['e', 'e', 'e', 'b', 'w', 'e', 'e', 'e'],
                ['e', 'e', 'e', 'e', 'e', 'e', 'e', 'e'],
                ['e', 'e', 'e', 'e', 'e', 'e', 'e', 'e'],
                ['e', 'e', 'e', 'e', 'e', 'e', 'e', 'e'],
            ],
        }
    }
}
