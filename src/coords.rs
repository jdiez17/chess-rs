use std::fmt;
use std::char;
use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Coords {
    pub x: usize,
    pub y: usize
}

impl Coords {
    pub fn file_try_from(file: char) -> Option<usize> {
        if file < 'a' || file > 'h' {
            return None;
        }

        Some((file as u32 - 'a' as u32) as usize)
    }

    pub fn rank_try_from(rank: char) -> Option<usize> {
        let rank_digit = rank.to_digit(10).unwrap();
        if rank_digit < 1 || rank_digit > 8 {
            return None;
        }

        Some((8 - rank_digit) as usize)
    }
}

impl<'a> TryFrom<&'a[char]> for Coords {
    type Error = String;

    fn try_from(s: &[char]) -> Result<Self, Self::Error> {
        if s.len() != 2 {
            return Err(format!("invalid length {}", s.len()));
        }

        match (Coords::file_try_from(s[0]), Coords::rank_try_from(s[1])) {
            (Some(x), Some(y)) => Ok(Coords { x: x, y: y }),
            _ => Err("invalid coords str".into())
        }
    }
}

impl fmt::Display for Coords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let file = char::from_u32(self.x as u32 + 'a' as u32).unwrap();
        let rank = (8 - self.y).to_string();

        write!(f, "{}{}", file, rank)
    }
}

