#![feature(try_from)]

mod piece;
mod square;
mod board;
mod coords;
mod op;

use board::Board;

fn main() {
    let board = Board::new();
    println!("Board:\n {}", board);
}
