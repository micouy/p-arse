//! String slices and characters.

use crate::{Error, Parser, Result};

impl<'b> Parser for &'b str {
    type Output = &'b str;

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
        let stripped = tail
            .strip_prefix(self)
            .ok_or_else(|| Error::expecting(format!("string '{}'", self)))?;

        Ok((self, stripped))
    }
}

impl Parser for char {
    type Output = char;

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
        let mut chars = tail.chars();
        let first = chars
            .next()
            .ok_or_else(|| Error::expecting(format!("char '{}'", self)))?;

        if first == *self {
            let tail = chars.as_str();

            Ok((first, tail))
        } else {
            Err(Error::expecting(format!("char '{}'", self)))
        }
    }
}

/// A [`Copy`] [`char`] range.
#[derive(Copy, Clone)]
pub struct CharRange {
    from: char,
    to: char,
}

/// Trait enabling a concise syntax for construction of [`CharRange`].
///
/// # Examples
///
/// ```
/// use p_arse::CharExt;
///
/// let a_to_z = 'a'.to('z');
/// ```
pub trait CharExt {
    fn to(self, to: char) -> CharRange;
}

impl CharExt for char {
    fn to(self, to: char) -> CharRange {
        CharRange { from: self, to }
    }
}

impl Parser for CharRange {
    type Output = char;

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
        let mut chars = tail.chars();
        let first = chars.next().ok_or_else(|| {
            Error::expecting(format!(
                "char from '{}' to '{}'",
                self.from, self.to
            ))
        })?;

        if (self.from..=self.to).contains(&first) {
            let tail = chars.as_str();

            Ok((first, tail))
        } else {
            Err(Error::expecting(format!(
                "char from '{}' to '{}'",
                self.from, self.to
            )))
        }
    }
}
