use crate::{Error, Parser, Result};

#[derive(Copy, Clone)]
pub struct Any {}

impl<'a> Parser<'a> for Any {
    type Output = char;

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        let mut chars = tail.chars();
        let first = chars.next().ok_or(Error {})?;
        let tail = chars.as_str();

        Ok((first, tail))
    }
}

pub fn any() -> Any {
    Any {}
}

#[derive(Copy, Clone)]
pub struct EndOfFile {}

impl<'a> Parser<'a> for EndOfFile {
    type Output = ();

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        if tail.is_empty() {
            Ok(((), tail))
        } else {
            Err(Error {})
        }
    }
}

pub fn eof() -> EndOfFile {
    EndOfFile {}
}
