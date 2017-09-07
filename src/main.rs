#![feature(try_from)]

mod traits;
mod piece;
mod square;
mod board;
mod coords;
mod op;

use traits::NewFromStr;
use board::Board;
use piece::Color;
use op::Op;

fn main() {
    let game = vec![
        ("e4", Color::White),
        ("d5", Color::Black),
        ("exd5", Color::White),
        ("Be6", Color::Black)
    ];

    let mut board = Board::default();
    println!("{}\n---", board);

    for (op_str, color) in game {
        let op = Op::new(op_str);
        board = board.apply(&op, &color).unwrap();

        println!("{:?} plays {} ({:?})", color, op_str, op);
        println!("{}", board);
        println!("---");
    }
}
