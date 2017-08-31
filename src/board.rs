use std::fmt;

use piece::{Piece, Color};
use sentence::Sentence;
use square::Square;
use coords::Coords;

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

    pub fn at(&self, coords: &Coords) -> Square {
        self.board[coords.y][coords.x]
    }

    pub fn set(&mut self, coords: &Coords, square: Square) {
        self.board[coords.y][coords.x] = square;
    }

    pub fn apply(&self, sentence: &Sentence, color: &Color) -> Result<Board, String> {
        let mut new_board = self.clone();

        match *sentence {
            Sentence::Move(ref m) => {
                let dst_sq = new_board.at(&m.dst);
                if dst_sq.piece != Piece::Empty {
                    return Err(format!("destination {} is not empty", m.dst));
                }

                let candidates: Vec<Coords> = match m.piece {
                    Piece::Pawn => {
                        (0..8).map(|y| {
                            Coords { x: m.dst.x, y: y }
                        })
                    },
                    _ => unimplemented!()
                }.filter(|coords| {
                    new_board.at(&coords).color == *color &&
                        self.moves(&coords, &sentence).contains(&m.dst)
                }).collect();

                if candidates.len() == 0 {
                    return Err(format!("no {:?} {:?} can reach destination {}", color, m.piece, m.dst));
                } else if candidates.len() > 1 {
                    return Err("move is ambiguous".to_owned());
                } else {
                    let from = &candidates[0];
                    let orig = new_board.at(&from);
                    new_board.set(&m.dst, orig);
                    new_board.set(&from, Square { piece: Piece::Empty, color: Color::None });
                }

            },
            Sentence::PawnCapture(ref pc) => {
                let dst_sq = new_board.at(&pc.dst);
                if dst_sq.piece == Piece::Empty {
                    return Err(format!("destination {} is empty", pc.dst));
                }
                if dst_sq.color == *color {
                    return Err(format!("cannot capture piece of own color at destination {}", pc.dst))
                }

                let candidates: Vec<Coords> = (0..8).map(|y| {
                    Coords { x: pc.src_x, y: y }
                }).filter(|coords| {
                    new_board.at(&coords).color == *color &&
                        self.moves(&coords, &sentence).contains(&pc.dst)
                }).collect();

                if candidates.len() == 0 {
                    return Err(format!("no {:?} {:?} can reach destination {}", color, Piece::Pawn, pc.dst));
                } else if candidates.len() > 1 {
                    return Err("move is ambiguous".to_owned());
                } else {
                    let from = &candidates[0];
                    let orig = new_board.at(&from);
                    new_board.set(&pc.dst, orig);
                    new_board.set(&from, Square { piece: Piece::Empty, color: Color::None });
                }
            }
        }

        Ok(new_board)
    }

    fn moves(&self, coords: &Coords, sentence: &Sentence) -> Vec<Coords> {
        let square = self.at(coords);
        let add = |dx, dy| {
            let mult = if square.color == Color::White { -1 } else { 1 };
            Coords { x: (coords.x as i32 + dx) as usize, y: (coords.y as i32 + dy * mult) as usize }
        };

        let mut res: Vec<Coords> = vec![];
        match square.piece {
            Piece::Pawn => {
                match *sentence {
                    Sentence::Move(_) => {
                        res.push(add(0, 1));

                        // Pawns can move two rows if they are at their starting position
                        if (square.color == Color::Black && coords.y == 1) ||
                               (square.color == Color::White && coords.y == 6) {
                            res.push(add(0, 2));
                        }

                    },
                    Sentence::PawnCapture(_) => {
                        // Pawns can move diagonally when capturing
                        res.push(add(-1, 1));
                        res.push(add(1, 1));
                    }
                }


            },
            Piece::Empty => {},
            _ => {}
        }

        res
    }

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

        Ok(()) // TODO: this is probably not a good idea
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