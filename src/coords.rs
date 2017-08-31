use std::fmt;
use std::char;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Coords {
    pub x: usize,
    pub y: usize
}

impl Coords {
    pub fn new(s: &str) -> Coords {
        Coords::parse(s).unwrap()
    }

    pub fn parse_file(file: char) -> Option<usize> {
        if file < 'a' || file > 'h' {
            return None;
        }

        Some((file as u32 - 'a' as u32) as usize)
    }

    pub fn parse_row(row: char) -> Option<usize> {
        let row_digit = row.to_digit(10).unwrap();
        if row_digit < 1 || row_digit > 8 {
            return None;
        }

        Some((8 - row_digit) as usize)
    }

    pub fn parse(s: &str) -> Option<Coords> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 2 {
            return None;
        }

        match (Coords::parse_file(chars[0]), Coords::parse_row(chars[1])) {
            (Some(x), Some(y)) => Some(Coords { x: x, y: y }),
            _ => None
        }
    }
}

impl fmt::Display for Coords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let file = char::from_u32(self.x as u32 + 'a' as u32).unwrap();
        let row = (self.y + 1).to_string();

        write!(f, "{}{}", file, row)
    }
}

