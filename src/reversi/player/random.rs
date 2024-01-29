use crate::reversi::board::{choice::Choice, point::Point};

use super::Player;

use rand::prelude::*;

impl Player {
    pub fn random(&self) -> Point {
        let choices: Vec<Choice> = self.board.search(self.side);

        let mut rand = rand::thread_rng();
        let num: usize = rand.gen_range(0..choices.len());

        let result: Point = choices[num].focus.copy();

        return result;
    }
}
