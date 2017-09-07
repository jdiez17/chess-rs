use std::convert::TryFrom;

use piece::Piece;
use coords::Coords;

#[derive(Debug, Eq, PartialEq)]
pub enum Op {
    Move { piece: Piece, dst: Coords },
    PawnCapture { src_x: usize, dst: Coords },
    DisambiguatedMove, /* TODO */
}

impl<'a> TryFrom<&'a[char]> for Op {
    type Error = String;

    fn try_from(s: &'a[char]) -> Result<Self, Self::Error> {
        let len = s.len();
        if len == 0 {
            return Err("empty op string".into());
        }

        if s[0].is_lowercase() {
            // This is a pawn move, pawn capture or pawn promotion.
            if len != 2 && len != 4 {
                return Err(format!("invalid op length {} for pawn move or capture",
                                   len));
            }

            if s[1] == 'x' {
                // This is a pawn capture.
                match (Coords::file_try_from(s[0]), Coords::try_from(&s[2..])) {
                    (Some(src_x), Ok(dst)) => {
                        return Ok(Op::PawnCapture { src_x: src_x, dst: dst })
                    },
                    (Some(_), Err(_)) => return Err("invalid destination".into()),
                    (None, Ok(_)) => return Err("invalid source file for pawn capture".into()),
                    _ => return Err("you did everything wrong, congrats".into())
                }
            } else if len == 2 {
                // This is a pawn move.
                match Coords::try_from(&s[0..]) {
                    Ok(dst) => {
                        return Ok(Op::Move { piece: Piece::Pawn, dst: dst });
                    }
                    Err(e) => return Err(format!("invalid destination: {}", e))
                }
            } else if len == 3 && s[2].is_uppercase() {
                // This is a pawn promotion.
                unimplemented!()
            }
        } else {
            // This is a move or capture,
            // TODO: disambiguating moves
            match (Piece::try_from(s[0]), Coords::try_from(&s[1..])) {
                (Ok(piece), Ok(dst)) => return Ok(Op::Move { piece: piece, dst: dst }),
                (_, _) => return Err("nope".into())
            }
        }

        return Err("unknown sentence string".into());
    }
}

#[cfg(test)]
mod tests {
    use traits::NewFromStr;
    use super::*;

    #[test]
    fn parse_pawn_move() {
        assert_eq!(Op::new("e4"), Op::Move { piece: Piece::Pawn, dst: Coords::new("e4") });
    }

    #[test]
    fn parse_pawn_capture() {
        assert_eq!(Op::new("exd5"), Op::PawnCapture { src_x: 4, dst: Coords::new("d5") });
    }

    #[test]
    fn test_regular_move() {
        assert_eq!(Op::new("Be4"), Op::Move { piece: Piece::Bishop, dst: Coords::new("e4") });
    }
}
