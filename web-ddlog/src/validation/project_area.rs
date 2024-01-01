#[derive(Debug)]
pub enum Error {
    TooShort,
    TooLong,
    MustContainChars
}

pub fn validate(s: &str) -> Option<Error> {
    match s.len() {
        0..=3 => return Some(Error::TooShort),
        8.. => return Some(Error::TooLong),
        _ => {},
    };
    if s.find(|ch: char| ch.is_alphabetic()).is_none() {
        return Some(Error::MustContainChars);
    };
    None
}