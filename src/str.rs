use crate::{pattern::pat, Parser, Result};

impl<'a> Parser<'a> for &str {
    type Output = &'a str;

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        pat(self).p_arse(tail)
    }
}
