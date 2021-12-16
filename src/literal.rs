//! String slices and characters.

use std::convert::Infallible;

use crate::{Error, Parser, Result};

impl<'a> Parser<'a, Infallible> for &str {
    type Output = &'a str;

    fn try_p_arse(
        &self,
        tail: &'a str,
    ) -> Result<'a, Self::Output, Infallible> {
        let stripped = tail
            .strip_prefix(self)
            .ok_or_else(|| Error::expecting(format!("string '{}'", self)))?;
        let len_diff = tail.len() - stripped.len();
        let head = &tail[0..len_diff];

        Ok((head, stripped))
    }
}

impl<'a> Parser<'a, Infallible> for char {
    type Output = char;

    fn try_p_arse(
        &self,
        tail: &'a str,
    ) -> Result<'a, Self::Output, Infallible> {
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

impl<'a> Parser<'a> for CharRange {
    type Output = char;

    fn try_p_arse(
        &self,
        tail: &'a str,
    ) -> Result<'a, Self::Output, Infallible> {
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
