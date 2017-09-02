use std::fmt;

use piece::{Piece, Color};
use square::Square;
//use coords::Coords;

#[derive(Debug, Clone)]
pub struct Board {
    board: [[Square; 8]; 8]
}

impl Board {
    pub fn new() -> Board {
        Board::parse(
            "♜♞♝♛♚♝♞♜\n\
             ♟♟♟♟♟♟♟♟\n\
                     \n\
                     \n\
                     \n\
                     \n\
             ♙♙♙♙♙♙♙♙\n\
             ♖♘♗♕♔♗♘♖"
        ).unwrap()
    }

    //pub fn at(&self, coords: &Coords) -> Square {
    //    self.board[coords.y][coords.x]
    //}

    //pub fn set(&mut self, coords: &Coords, square: Square) {
    //    self.board[coords.y][coords.x] = square;
    //}

    fn parse(s: &str) -> Result<Board, String> {
        let lines: Vec<&str> = s.split("\n").collect();
        let count = lines.len();
        if lines.len() != 8 {
            return Err(format!("invalid number of lines, got {}, expecting 8", count));
        }

        let mut board: [[Square; 8]; 8] = [[Square { piece: Piece::Empty, color: Color::None }; 8]; 8];

        for (i, line) in lines.into_iter().enumerate() {
            let chars: Vec<char> = line.chars().collect();
            let count = chars.len();
            if count != 8 {
                return Err(format!("invalid number of chars in line {}, got {}, expecting 8", i, count));
            }
            for (ii, c) in chars.into_iter().enumerate() {
                if let Some(sq) = Square::parse(c) {
                    board[i][ii] = sq;
                } else {
                    return Err(format!("cannot parse char '{}' at position {} in line {}", c, ii, i));
                }
            }
        }

        Ok(Board { board: board })
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "  a b c d e f g h\n")?;

        for i in 0..8 {
            write!(f, "{}", 8 - i)?;
            for ii in 0..8 {
                write!(f, " {}", self.board[i][ii])?;
            }
            write!(f, " {}\n", 8 - i)?;
        }

        write!(f, "  a b c d e f g h")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_invalid_num_lines() {
        match Board::parse("♜♞♝♛♚♝♞♜\n") {
            Ok(_) => assert!(false),
            Err(msg) => assert!(msg.contains("invalid number of lines"))
        }
    }

    #[test]
    fn parse_invalid_chars() {
        let result = Board::parse(
            "♜♞♝♛♚♝♞♜\n\
             ♟♟♟♟♟♟♟♟\n\
                     \n\
                     \n\
                     \n\
                     \n\
             ♙x♙♙♙♙♙♙\n\
             ♖♘♗♕♔♗♘♖"
        );
        match result {
            Ok(_) => assert!(false),
            Err(msg) => assert!(msg.contains("cannot parse char"))
        }
    }

    #[test]
    fn parse_ok() {
        let result = Board::parse(
            "♜♞♝♛♚♝♞♜\n\
             ♟♟♟♟♟♟♟♟\n\
                     \n\
                     \n\
                     \n\
                     \n\
             ♙♙♙♙♙♙♙♙\n\
             ♖♘♗♕♔♗♘♖"
        );

        match result {
            Err(_) => assert!(false),
            Ok(_) => { /* TODO verify board */ }
        }
    }
}
