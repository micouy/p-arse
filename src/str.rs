use crate::{pattern::pat, Error, Parser, Result};

impl<'a> Parser<'a> for &str {
    type Output = &'a str;

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        pat(self).p_arse(tail)
    }
}

impl<'a> Parser<'a> for char {
    type Output = char;

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        let mut chars = tail.chars();
        let first = chars.next().ok_or(Error {})?;

        if first == *self {
            let tail = chars.as_str();

            Ok((first, tail))
        } else {
            Err(Error {})
        }
    }
}

#[derive(Copy, Clone)]
pub struct CharRange {
    from: char,
    to: char,
}

pub trait CharExt {
    fn to(self, to: char) -> CharRange;
}

impl CharExt for char {
    fn to(self, to: char) -> CharRange {
        CharRange { from: self, to }
    }
}

impl<'a> Parser<'a> for CharRange {
    type Output = char;

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        let mut chars = tail.chars();
        let first = chars.next().ok_or(Error {})?;

        if (self.from..=self.to).contains(&first) {
            let tail = chars.as_str();

            Ok((first, tail))
        } else {
            Err(Error {})
        }
    }
}
