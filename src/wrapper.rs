use std::marker::PhantomData;

use crate::{Error, Parser, Result};

#[derive(Copy, Clone)]
pub struct ZeroOrMore<'a, P>
where
    P: Parser<'a>,
{
    pub(crate) parser: P,
    pub(crate) marker: PhantomData<&'a ()>,
}

impl<'a, P> Parser<'a> for ZeroOrMore<'a, P>
where
    P: Parser<'a>,
{
    type Output = Vec<P::Output>;

    fn p_arse(&self, mut tail: &'a str) -> Result<'a, Self::Output> {
        let mut output = vec![];

        while let Ok((output_i, tail_i)) = self.parser.p_arse(tail) {
            tail = tail_i;
            output.push(output_i);
        }

        Ok((output, tail))
    }
}

#[derive(Copy, Clone)]
pub struct OneOrMore<'a, P>
where
    P: Parser<'a>,
{
    pub(crate) parser: P,
    pub(crate) marker: PhantomData<&'a ()>,
}

impl<'a, P> Parser<'a> for OneOrMore<'a, P>
where
    P: Parser<'a>,
{
    type Output = Vec<P::Output>;

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        let (first, tail) = self.parser.p_arse(tail)?;

        match (&self.parser).zore().p_arse(tail) {
            Ok((mut rest, tail)) => {
                rest.insert(0, first);

                Ok((rest, tail))
            }
            Err(_) => Ok((vec![first], tail)),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Ignorant<'a, P>
where
    P: Parser<'a>,
{
    pub(crate) parser: P,
    pub(crate) marker: PhantomData<&'a ()>,
}

impl<'a, P> Parser<'a> for Ignorant<'a, P>
where
    P: Parser<'a>,
{
    type Output = ();

    fn p_arse(&self, tail: &'a str) -> Result<'a, ()> {
        self.parser.p_arse(tail).map(|(_, tail)| ((), tail))
    }
}

#[derive(Copy, Clone)]
pub struct Opt<'a, P>
where
    P: Parser<'a>,
{
    pub(crate) parser: P,
    pub(crate) marker: PhantomData<&'a ()>,
}

impl<'a, P> Parser<'a> for Opt<'a, P>
where
    P: Parser<'a>,
{
    type Output = Option<P::Output>;

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        if let Ok((output, tail)) = self.parser.p_arse(tail) {
            Ok((Some(output), tail))
        } else {
            Ok((None, tail))
        }
    }
}

// Manual derivation of `Copy` and `Clone`.
pub struct Map<'a, P, F, U>
where
    P: Parser<'a>,
    F: Fn(P::Output) -> U + Clone,
{
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) lifetime_marker: PhantomData<&'a ()>,
    pub(crate) u_marker: PhantomData<U>,
}

impl<'a, P, F, U> Clone for Map<'a, P, F, U>
where
    P: Parser<'a>,
    F: Fn(P::Output) -> U + Clone,
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            f: self.f.clone(),
            lifetime_marker: self.lifetime_marker,
            u_marker: PhantomData,
        }
    }
}

impl<'a, P, F, U> Copy for Map<'a, P, F, U>
where
    P: Parser<'a> + Copy,
    F: Fn(P::Output) -> U + Copy,
{
}

impl<'a, P, F, U> Parser<'a> for Map<'a, P, F, U>
where
    F: Fn(P::Output) -> U + Clone,
    P: Parser<'a>,
{
    type Output = U;

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        self.parser
            .p_arse(tail)
            .map(|(t, tail)| ((self.f)(t), tail))
    }
}

#[derive(Copy, Clone)]
pub struct Or<'a, P0, P1>
where
    P0: Parser<'a>,
    P1: Parser<'a, Output = P0::Output>,
{
    pub(crate) parser_0: P0,
    pub(crate) parser_1: P1,
    pub(crate) marker: PhantomData<&'a ()>,
}

impl<'a, P0, P1> Parser<'a> for Or<'a, P0, P1>
where
    P0: Parser<'a>,
    P1: Parser<'a, Output = P0::Output>,
{
    type Output = P0::Output;

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        if let Ok((output, tail)) = self.parser_0.p_arse(tail) {
            Ok((output, tail))
        } else {
            self.parser_1.p_arse(tail)
        }
    }
}

#[derive(Copy, Clone)]
pub struct NegativeLookahead<'a, P>
where
    P: Parser<'a>,
{
    pub(crate) parser: P,
    pub(crate) marker: PhantomData<&'a ()>,
}

impl<'a, P> Parser<'a> for NegativeLookahead<'a, P>
where
    P: Parser<'a>,
{
    type Output = ();

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        if self.parser.p_arse(tail).is_err() {
            Ok(((), tail))
        } else {
            // TODO what to do here?
            Err(Error::expecting("negative lookahead"))
        }
    }
}

#[derive(Copy, Clone)]
pub struct PositiveLookahead<'a, P>
where
    P: Parser<'a>,
{
    pub(crate) parser: P,
    pub(crate) marker: PhantomData<&'a ()>,
}

impl<'a, P> Parser<'a> for PositiveLookahead<'a, P>
where
    P: Parser<'a>,
{
    type Output = ();

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        if self.parser.p_arse(tail).is_ok() {
            Ok(((), tail))
        } else {
            // TODO what to do here?
            Err(Error::expecting("positive lookahead"))
        }
    }
}

#[derive(Clone)]
pub struct Named<'a, P>
where
    P: Parser<'a>,
{
    pub(crate) parser: P,
    pub(crate) marker: PhantomData<&'a ()>,
    pub(crate) name: String,
}

impl<'a, P> Parser<'a> for Named<'a, P>
where
    P: Parser<'a>,
{
    type Output = P::Output;

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        self.parser
            .p_arse(tail)
            .map_err(|err| err.push(self.name.clone()))
    }
}
