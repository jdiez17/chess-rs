mod piece;
mod square;
mod sentence;
mod board;
mod coords;

use sentence::Sentence;
use board::Board;
use piece::Color;

fn main() {
    let game = vec![
        ("e4", Color::White),
        ("d5", Color::Black),
        ("exd5", Color::White)
    ];

    let mut board = Board::new();
    println!("{}", board);
    println!("---");

    for (op_str, color) in game {
        let sentence = Sentence::parse(op_str).unwrap();
        println!("{:?} plays {} ({:?}):", color, op_str, &sentence);
        println!();

        board = board.apply(&sentence, &color).unwrap();
        println!("{}", board);
        println!("---");
    }
}
