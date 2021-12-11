use std::marker::PhantomData;

use crate::{Error, Parser, Result};

#[derive(Copy, Clone)]
pub struct Pattern<'a, P>
where
    P: std::str::pattern::Pattern<'a> + Clone,
{
    pattern: P,
    marker: PhantomData<&'a ()>,
}

impl<'a, P> Parser<'a> for Pattern<'a, P>
where
    P: std::str::pattern::Pattern<'a> + Clone,
{
    type Output = &'a str;

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        let stripped =
            tail.strip_prefix(self.pattern.clone()).ok_or(Error {})?;
        let len_diff = tail.len() - stripped.len();
        let head = &tail[0..len_diff];

        Ok((head, stripped))
    }
}

pub fn pat<'a, P>(pattern: P) -> Pattern<'a, P>
where
    P: std::str::pattern::Pattern<'a> + Clone,
{
    Pattern {
        pattern,
        marker: PhantomData,
    }
}
