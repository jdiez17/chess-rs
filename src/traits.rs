use std::convert::TryFrom;

pub trait NewFromStr {
    fn new(s: &str) -> Self;
}

impl<T> NewFromStr for T
    where T: for <'a> TryFrom<&'a[char], Error=String> {

    fn new(s: &str) -> Self {
        let chars: Vec<char> = s.chars().collect();
        Self::try_from(&chars[..]).unwrap()
    }
}
