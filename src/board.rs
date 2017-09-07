use std::fmt;
use std::str::FromStr;
use std::convert::TryFrom;

use piece::{Piece, Color};
use coords::Coords;
use square::Square;
use op::Op;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Board {
    board: [[Square; 8]; 8]
}

impl Board {
    pub fn new(s: &str) -> Board {
        Board::from_str(s).unwrap()
    }

    pub fn default() -> Board {
        Board::new(
            "♜♞♝♛♚♝♞♜\n\
             ♟♟♟♟♟♟♟♟\n\
                     \n\
                     \n\
                     \n\
                     \n\
             ♙♙♙♙♙♙♙♙\n\
             ♖♘♗♕♔♗♘♖"
        )
    }

    pub fn at(&self, coords: &Coords) -> Square {
        self.board[coords.y][coords.x]
    }

    pub fn set(&mut self, coords: &Coords, square: Square) {
        self.board[coords.y][coords.x] = square;
    }

    fn apply_move(&mut self, mut candidates: Vec<Coords>, dst: &Coords,
                  piece: &Piece, color: &Color) -> Result<(), String> {

        // The destination square must be empty.
        if self.at(dst).piece != Piece::Empty {
            return Err(format!("destination square {} is not empty", dst));
        }

        // The candidate coordinates must contain a square of the
        // correct color and correct piece.
        candidates.retain(|c| {
            self.at(c) == Square { piece: *piece, color: *color }

        });

        if candidates.len() == 0 {
            return Err(format!("move is invalid, no {:?} {:?} can reach {}", color,
                               piece, dst));
        }
        if candidates.len() > 1 {
            return Err("move is ambiguous".into());
        }

        let src = &candidates[0];
        let at_src = self.at(&src);
        self.set(&dst, at_src);
        self.set(&src, Square { piece: Piece::Empty, color: Color::None });

        Ok(())
    }

    fn apply_capture(&mut self, mut candidates: Vec<Coords>, dst: &Coords,
                     piece: &Piece, color: &Color) -> Result<(), String> {

        // The destination square must contain a piece of the opposite color.
        if self.at(dst).color != color.opposite() {
            return Err(format!("destination square {} is not of the opposite color", dst));
        }

        // The candidate coordinates must contain a square of the
        // coorrect color and correct piece.
        candidates.retain(|c| {
            self.at(c) == Square { piece: *piece, color: *color }
        });

        if candidates.len() == 0 {
            return Err(format!("capture is invalid, no {:?} {:?} can capture {}",
                               color, piece, dst));
        }
        if candidates.len() > 1 {
            return Err("capture is ambiguous".into());
        }

        let src = &candidates[0];
        let at_src = self.at(&src);
        self.set(&dst, at_src);
        self.set(src, Square { piece: Piece::Empty, color: Color::None });

        Ok(())
    }

    pub fn apply(&self, op: &Op, color: &Color) -> Result<Board, String> {
        let mut result = self.clone();
        if *color == Color::None {
            return Err("invalid color None".into());
        }

        match *op {
            Op::Move { ref piece, ref dst } => {
                let candidates: Vec<Coords> = match *piece {
                    Piece::Pawn => {
                        (0..8).map(|y| {
                            Coords { x: dst.x, y: y }
                        }).filter(|c| {
                            // Only pawns at their starting position can move
                            // two squares.
                            if dst.y == 3 {
                                return c.y == 1;
                            }
                            if dst.y == 4 {
                                return c.y == 6;
                            }
                            // Otherwise, pawns can only move one square.
                            (c.y as i8 - dst.y as i8).abs() == 1
                        }).collect()
                    },
                    Piece::Bishop => {
                        // TODO: blocking pieces, 4 directions
                        (0..8).map(|diff| {
                            let x = dst.x as i8 - diff;
                            let y = dst.y as i8 - diff;

                            (x, y)
                        }).filter(|&(x, y)| {
                            x >= 0 && y >= 0
                        }).map(|(x, y)| {
                            Coords { x: x as usize, y: y as usize}
                        }).collect()
                    }
                    _ => unimplemented!()
                };

                println!("candidates: {:?}", candidates);

                result.apply_move(candidates, dst, piece, color)?;
            }
            Op::PawnCapture { src_x, ref dst } => {
                /* TODO: e.p. captures */
                let mut candidates = vec![];
                let y = dst.y as i8 + 1 * if *color == Color::White { 1 } else { -1 };
                if y < 8 && y >= 0 {
                    if dst.x > 0 {
                        candidates.push(Coords { x: dst.x - 1, y: y as usize });
                    }
                    if dst.x < 7 {
                        candidates.push(Coords { x: dst.x + 1, y: y as usize });
                    }
                }

                // The source file must match.
                candidates.retain(|c| {
                    c.x == src_x
                });

                result.apply_capture(candidates, dst, &Piece::Pawn, color)?;
            }
            _ => unimplemented!()
        }

        Ok(result)
    }
}

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
                if let Ok(sq) = Square::try_from(c) {
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
    use traits::NewFromStr;
    use super::*;

    #[test]
    fn from_str() {
        assert!(Board::from_str(
            "♜♞♝♛♚♝♞♜\n\
             ♟♟♟♟♟♟♟♟\n\
                     \n\
                     \n\
                     \n\
                     \n\
             ♙♙♙♙♙♙♙♙\n\
             ♖♘♗♕♔♗♘♖"
        ).is_ok());
    }

    #[test]
    fn apply_bishop_move() {
        let op = Op::new("Be6");
        let color = Color::Black;
        let board = Board::new(
            "♜♞♝♛♚♝♞♜\n\
             ♟♟♟ ♟♟♟♟\n\
                     \n\
                ♟    \n\
                     \n\
                     \n\
             ♙♙♙♙♙♙♙♙\n\
             ♖♘♗♕♔♗♘♖"
        );
        let expect = Board::new(
            "♜♞ ♛♚♝♞♜\n\
             ♟♟♟ ♟♟♟♟\n\
                 ♝   \n\
                ♟    \n\
                     \n\
                     \n\
             ♙♙♙♙♙♙♙♙\n\
             ♖♘♗♕♔♗♘♖"
        );

        assert_eq!(board.apply(&op, &color), Ok(expect));
    }

}
