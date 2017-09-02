use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    Empty
}

impl TryFrom<char> for Piece {
    type Err = ();

    fn try_from(c: char) -> Result<Self, Self::Err> {
        match c {
            'K' => Ok(Piece::King),
            'Q' => Ok(Piece::Queen),
            'R' => Ok(Piece::Rook),
            'B' => Ok(Piece::Bishop),
            'N' => Ok(Piece::Knight),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn piece_try_from() {
        assert_eq!(Piece::try_from('K'), Ok(Piece::King));
        assert!(Piece::try_from('Z').is_err());
    }
}
