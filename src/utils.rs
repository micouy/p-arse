#![allow(non_camel_case_types)]

//! Basic utilities.

use crate::{parser::Parser, Error, Result};

/// A [`Parser`] matching any single character.
#[derive(Copy, Clone)]
pub struct any();

impl<'a, E> Parser<'a, E> for any {
    type Output = char;

    fn try_p_arse(&self, tail: &'a str) -> Result<'a, Self::Output, E> {
        let mut chars = tail.chars();
        let first = chars.next().ok_or_else(|| Error::expecting("any"))?;
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
/// use p_arse::{traits::*, eoi};
///
/// let without_eoi = "abc".zore();
/// assert!(without_eoi.p_arse("abcabcxxx").is_ok());
///
/// let with_eoi = ("abc".zore(), eoi());
/// assert!(with_eoi.p_arse("abcabcxxx").is_err());
/// ```
#[derive(Copy, Clone)]
pub struct eoi();

impl<'a, E> Parser<'a, E> for eoi {
    type Output = ();

    fn try_p_arse(&self, tail: &'a str) -> Result<'a, Self::Output, E> {
        if tail.is_empty() {
            Ok(((), tail))
        } else {
            Err(Error::expecting("eoi"))
        }
    }
}
