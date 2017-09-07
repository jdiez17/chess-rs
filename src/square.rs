use std::fmt;
use std::char;
use std::convert::TryFrom;

use piece::{Piece, Color};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Square {
    pub piece: Piece,
    pub color: Color
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let offset = if self.color == Color::Black { '♚' as u32 - '♔' as u32 } else { 0 };
        let symbol = char::from_u32(offset + match self.piece {
            Piece::King => '♔',
            Piece::Queen => '♕',
            Piece::Rook => '♖',
            Piece::Bishop => '♗',
            Piece::Knight => '♘',
            Piece::Pawn => '♙',
            Piece::Empty => ' '
        } as u32).unwrap();

        write!(f, "{}", symbol)
    }
}

impl TryFrom<char> for Square {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        if c == ' ' {
            return Ok(Square { piece: Piece::Empty, color: Color::None });
        }
        if c < '♔' || c > '♟' {
            return Err(());
        }

        let color = if c >= '♚' { Color::Black } else { Color::White };
        let offset = if color == Color::Black { '♚' as u32 - '♔' as u32 } else { 0 };
        let piece = match char::from_u32(c as u32 - offset).unwrap() {
            '♔' => Piece::King,
            '♕' => Piece::Queen,
            '♖' => Piece::Rook,
            '♗' => Piece::Bishop,
            '♘' => Piece::Knight,
            '♙' => Piece::Pawn,
            _ => return Err(())
        };

        Ok(Square { piece: piece, color: color })
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_square() {
        match Square::try_from('♜') {
            Ok(sq) => {
                assert_eq!(sq.piece, Piece::Rook);
                assert_eq!(sq.color, Color::Black);
            }
            Err(_) => assert!(false)
        }
        match Square::try_from('♙') {
            Ok(sq) => {
                assert_eq!(sq.piece, Piece::Pawn);
                assert_eq!(sq.color, Color::White);
            }
            Err(_) => assert!(false)
        }
    }
}
