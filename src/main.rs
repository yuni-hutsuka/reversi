// main

mod attachment;
mod logic;
mod reversi;

use logic::logic;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input: String = args[1].to_owned();

    let output: String = logic(input);

    println!("{}", output);
}
