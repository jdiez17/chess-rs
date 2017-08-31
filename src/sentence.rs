use piece::Piece;
use coords::Coords;

#[derive(Debug)]
pub struct Move {
    pub piece: Piece,
    pub dst: Coords
}

#[derive(Debug)]
pub struct PawnCapture {
    pub dst: Coords,
    pub src_x: usize
}

#[derive(Debug)]
pub enum Sentence {
    Move(Move),
    PawnCapture(PawnCapture),
}

impl Sentence {
    pub fn new(s: &str) -> Sentence {
        Sentence::parse(s).unwrap()
    }

    pub fn parse(s: &str) -> Result<Sentence, String> {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        if len == 0 {
            return Err("empty sentence string".to_owned());
        }
        let first = chars[0];
        if first.is_lowercase() {
            // pawn move or capture
            if len != 2 && len != 4 {
                return Err(format!("invalid sentence length {} for pawn move or capture", len));
            }

            if chars[1] == 'x' {
                // this is a pawn capture
                match (Coords::parse_file(chars[0]), Coords::parse(&s[2..])) {
                    (Some(src_x), Some(dst)) => {
                        return Ok(Sentence::PawnCapture(PawnCapture {
                            dst: dst,
                            src_x: src_x
                        }))
                    },
                    (Some(_), None) => return Err("invalid destination".to_owned()),
                    (None, Some(_)) => return Err("invalid source file for pawn capture".to_owned()),
                    _ => return Err("you did everything wrong, congrats".to_owned())
                }
            } else {
                // this is a pawn move
                match Coords::parse(&s[0..]) {
                    Some(dst) => {
                        return Ok(Sentence::Move(Move {
                            piece: Piece::Pawn,
                            dst: dst
                        }));
                    },
                    None => {
                        return Err("invalid destination".to_owned());
                    }
                }
            }

        }

        Err("unknown sentence string".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pawn_move() {
        match Sentence::parse("e4").unwrap() {
            Sentence::Move(m) => {
                assert_eq!(m.piece, Piece::Pawn);
                assert_eq!(m.dst, Coords::new("e4"));
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn parse_invalid_pawn_move() {
        match Sentence::parse("e9") {
            Ok(_) => assert!(false),
            Err(msg) => assert!(msg.contains("invalid destination"))
        }
    }

    #[test]
    fn parse_pawn_capture() {
        match Sentence::parse("exd5").unwrap() {
            Sentence::PawnCapture(pc) => {
                assert_eq!(pc.dst, Coords::new("d5"));
                assert_eq!(pc.src_x, 4);
            }
            _ => assert!(false)
        }
    }
}
