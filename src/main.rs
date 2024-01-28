// main

mod reversi;

use reversi::board::Board;

fn main() {
    let board: Board = Board::new();

    board.print();
    println!("{:?}", board.search('b'));
}
