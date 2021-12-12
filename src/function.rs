//! Implementation of [`Parser`] for functions.

use crate::{Parser, Result};

impl<'a, F, T> Parser<'a> for F
where
    F: Fn(&'a str) -> Result<'a, T> + Clone,
{
    type Output = T;

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        self(tail)
    }
}
