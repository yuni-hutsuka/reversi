use super::Player;

use rand::prelude::*;

impl Player {
    pub fn random(&self) -> (usize, usize) {
        let choices: Vec<((usize, usize), (usize, usize), usize)> = self.board.search(self.side);

        let mut rand = rand::thread_rng();
        let num: usize = rand.gen_range(0..choices.len());

        let result = choices[num].0;

        return result;
    }
}