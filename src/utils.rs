#![allow(non_camel_case_types)]

//! Basic utilities.

use crate::{parser::Parser, Error, Result};

/// A [`Parser`] matching any single character.
#[derive(Copy, Clone)]
pub struct any();

impl Parser for any {
    type Output = char;

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
        let mut chars = tail.chars();
        let first =
            chars.next().ok_or_else(|| Error::expecting("any", tail))?;
        let tail = chars.as_str();

        Ok((first, tail))
    }
}

/// A [`Parser`] matching the end of input. Used to ensure that the whole input
/// has matched, i.e. when looking for zero or more repetitions.
///
/// # Examples
///
/// ```
/// use p_arse::{Parser, eoi};
///
/// let without_eoi = "abc".zore();
/// assert!(without_eoi.p_arse("abcabcxxx").is_ok());
///
/// let with_eoi = ("abc".zore(), eoi());
/// assert!(with_eoi.p_arse("abcabcxxx").is_err());
/// ```
#[derive(Copy, Clone)]
pub struct eoi();

impl Parser for eoi {
    type Output = ();

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
        if tail.is_empty() {
            Ok(((), tail))
        } else {
            Err(Error::expecting("eoi", tail))
        }
    }
}
