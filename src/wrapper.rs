use std::{convert::Infallible, marker::PhantomData};

use crate::{Error, Parser, Result};

pub struct ZeroOrMore<'a, P, E = Infallible>
where
    P: Parser<'a, E>,
{
    pub(crate) parser: P,
    pub(crate) marker: PhantomData<(&'a (), E)>,
}

impl<'a, P, E> Clone for ZeroOrMore<'a, P, E>
where
    P: Parser<'a, E>,
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser,
            marker: self.marker,
        }
    }
}

impl<'a, P, E> Copy for ZeroOrMore<'a, P, E> where P: Parser<'a, E> {}

impl<'a, P, E> Parser<'a, E> for ZeroOrMore<'a, P, E>
where
    P: Parser<'a, E>,
{
    type Output = Vec<P::Output>;

    fn try_p_arse(&self, mut tail: &'a str) -> Result<'a, Self::Output, E> {
        let mut output = vec![];

        while let Ok((output_i, tail_i)) = self.parser.try_p_arse(tail) {
            tail = tail_i;
            output.push(output_i);
        }

        Ok((output, tail))
    }
}

pub struct OneOrMore<'a, P, E = Infallible>
where
    P: Parser<'a, E>,
{
    pub(crate) parser: P,
    pub(crate) marker: PhantomData<(&'a (), E)>,
}

impl<'a, P, E> Clone for OneOrMore<'a, P, E>
where
    P: Parser<'a, E>,
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser,
            marker: self.marker,
        }
    }
}

impl<'a, P, E> Copy for OneOrMore<'a, P, E> where P: Parser<'a, E> {}

impl<'a, P, E> Parser<'a, E> for OneOrMore<'a, P, E>
where
    P: Parser<'a, E>,
{
    type Output = Vec<P::Output>;

    fn try_p_arse(&self, tail: &'a str) -> Result<'a, Self::Output, E> {
        let (first, tail) = self.parser.try_p_arse(tail)?;

        match (&self.parser).zore().try_p_arse(tail) {
            Ok((mut rest, tail)) => {
                rest.insert(0, first);

                Ok((rest, tail))
            }
            Err(_) => Ok((vec![first], tail)),
        }
    }
}

pub struct Ignorant<'a, P, E = Infallible>
where
    P: Parser<'a, E>,
{
    pub(crate) parser: P,
    pub(crate) marker: PhantomData<(&'a (), E)>,
}

impl<'a, P, E> Clone for Ignorant<'a, P, E>
where
    P: Parser<'a, E>,
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser,
            marker: self.marker,
        }
    }
}

impl<'a, P, E> Copy for Ignorant<'a, P, E> where P: Parser<'a, E> {}

impl<'a, P, E> Parser<'a, E> for Ignorant<'a, P, E>
where
    P: Parser<'a, E>,
{
    type Output = ();

    fn try_p_arse(&self, tail: &'a str) -> Result<'a, Self::Output, E> {
        self.parser.try_p_arse(tail).map(|(_, tail)| ((), tail))
    }
}

pub struct Opt<'a, P, E = Infallible>
where
    P: Parser<'a, E>,
{
    pub(crate) parser: P,
    pub(crate) marker: PhantomData<(&'a (), E)>,
}

impl<'a, P, E> Clone for Opt<'a, P, E>
where
    P: Parser<'a, E>,
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser,
            marker: self.marker,
        }
    }
}

impl<'a, P, E> Copy for Opt<'a, P, E> where P: Parser<'a, E> {}

impl<'a, P, E> Parser<'a, E> for Opt<'a, P, E>
where
    P: Parser<'a, E>,
{
    type Output = Option<P::Output>;

    fn try_p_arse(&self, tail: &'a str) -> Result<'a, Self::Output, E> {
        if let Ok((output, tail)) = self.parser.try_p_arse(tail) {
            Ok((Some(output), tail))
        } else {
            Ok((None, tail))
        }
    }
}

pub struct Map<'a, P, F, U, E = Infallible>
where
    P: Parser<'a, E>,
    F: Fn(P::Output) -> U + Copy,
{
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) marker: PhantomData<(&'a (), U, E)>,
}

impl<'a, P, F, U, E> Clone for Map<'a, P, F, U, E>
where
    P: Parser<'a, E>,
    F: Fn(P::Output) -> U + Copy,
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser,
            f: self.f,
            marker: self.marker,
        }
    }
}

impl<'a, P, F, U, E> Copy for Map<'a, P, F, U, E>
where
    P: Parser<'a, E> + Copy,
    F: Fn(P::Output) -> U + Copy,
{
}

impl<'a, P, F, U, E> Parser<'a, E> for Map<'a, P, F, U, E>
where
    F: Fn(P::Output) -> U + Copy,
    P: Parser<'a, E>,
{
    type Output = U;

    fn try_p_arse(&self, tail: &'a str) -> Result<'a, Self::Output, E> {
        self.parser
            .try_p_arse(tail)
            .map(|(t, tail)| ((self.f)(t), tail))
    }
}

pub struct AndThen<'a, P, F, U, E = Infallible>
where
    P: Parser<'a, E>,
    F: Fn(P::Output) -> std::result::Result<U, E> + Copy,
{
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) marker: PhantomData<(&'a (), U, E)>,
}

impl<'a, P, F, U, E> Clone for AndThen<'a, P, F, U, E>
where
    P: Parser<'a, E>,
    F: Fn(P::Output) -> std::result::Result<U, E> + Copy,
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser,
            f: self.f,
            marker: self.marker,
        }
    }
}

impl<'a, P, F, U, E> Copy for AndThen<'a, P, F, U, E>
where
    P: Parser<'a, E> + Copy,
    F: Fn(P::Output) -> std::result::Result<U, E> + Copy,
{
}

impl<'a, P, F, U, E> Parser<'a, E> for AndThen<'a, P, F, U, E>
where
    P: Parser<'a, E>,
    F: Fn(P::Output) -> std::result::Result<U, E> + Copy,
{
    type Output = U;

    fn try_p_arse(&self, tail: &'a str) -> Result<'a, Self::Output, E> {
        self.parser.try_p_arse(tail).and_then(|(t, tail)| {
            let u = (self.f)(t).map_err(Error::user)?;

            Ok((u, tail))
        })
    }
}

pub struct Or<'a, P0, P1, E = Infallible>
where
    P0: Parser<'a, E>,
    P1: Parser<'a, E, Output = P0::Output>,
{
    pub(crate) parser_0: P0,
    pub(crate) parser_1: P1,
    pub(crate) marker: PhantomData<(&'a (), E)>,
}

impl<'a, P0, P1, E> Clone for Or<'a, P0, P1, E>
where
    P0: Parser<'a, E>,
    P1: Parser<'a, E, Output = P0::Output>,
{
    fn clone(&self) -> Self {
        Self {
            parser_0: self.parser_0,
            parser_1: self.parser_1,
            marker: self.marker,
        }
    }
}

impl<'a, P0, P1, E> Copy for Or<'a, P0, P1, E>
where
    P0: Parser<'a, E>,
    P1: Parser<'a, E, Output = P0::Output>,
{
}

impl<'a, P0, P1, E> Parser<'a, E> for Or<'a, P0, P1, E>
where
    P0: Parser<'a, E>,
    P1: Parser<'a, E, Output = P0::Output>,
{
    type Output = P0::Output;

    fn try_p_arse(&self, tail: &'a str) -> Result<'a, Self::Output, E> {
        if let Ok((output, tail)) = self.parser_0.try_p_arse(tail) {
            Ok((output, tail))
        } else {
            self.parser_1.try_p_arse(tail)
        }
    }
}

pub struct NegativeLookahead<'a, P, E = Infallible>
where
    P: Parser<'a, E>,
{
    pub(crate) parser: P,
    pub(crate) marker: PhantomData<(&'a (), E)>,
}

impl<'a, P, E> Clone for NegativeLookahead<'a, P, E>
where
    P: Parser<'a, E>,
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser,
            marker: self.marker,
        }
    }
}

impl<'a, P, E> Copy for NegativeLookahead<'a, P, E> where P: Parser<'a, E> {}

impl<'a, P, E> Parser<'a, E> for NegativeLookahead<'a, P, E>
where
    P: Parser<'a, E>,
{
    type Output = ();

    fn try_p_arse(&self, tail: &'a str) -> Result<'a, Self::Output, E> {
        if self.parser.try_p_arse(tail).is_err() {
            Ok(((), tail))
        } else {
            // TODO what to put here?
            Err(Error::expecting("negative lookahead"))
        }
    }
}

pub struct PositiveLookahead<'a, P, E = Infallible>
where
    P: Parser<'a, E>,
{
    pub(crate) parser: P,
    pub(crate) marker: PhantomData<(&'a (), E)>,
}

impl<'a, P, E> Clone for PositiveLookahead<'a, P, E>
where
    P: Parser<'a, E>,
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser,
            marker: self.marker,
        }
    }
}

impl<'a, P, E> Copy for PositiveLookahead<'a, P, E> where P: Parser<'a, E> {}

impl<'a, P, E> Parser<'a, E> for PositiveLookahead<'a, P, E>
where
    P: Parser<'a, E>,
{
    type Output = ();

    fn try_p_arse(&self, tail: &'a str) -> Result<'a, Self::Output, E> {
        if self.parser.try_p_arse(tail).is_ok() {
            Ok(((), tail))
        } else {
            // TODO what to put here?
            Err(Error::expecting("positive lookahead"))
        }
    }
}

pub struct Named<'a, P, E = Infallible>
where
    P: Parser<'a, E>,
{
    pub(crate) parser: P,
    pub(crate) name: &'static str,
    pub(crate) marker: PhantomData<(&'a (), E)>,
}

impl<'a, P, E> Clone for Named<'a, P, E>
where
    P: Parser<'a, E>,
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser,
            marker: self.marker,
            name: self.name,
        }
    }
}

impl<'a, P, E> Copy for Named<'a, P, E> where P: Parser<'a, E> {}

impl<'a, P, E> Parser<'a, E> for Named<'a, P, E>
where
    P: Parser<'a, E>,
{
    type Output = P::Output;

    fn try_p_arse(&self, tail: &'a str) -> Result<'a, Self::Output, E> {
        self.parser
            .try_p_arse(tail)
            .map_err(|err| err.push(self.name))
    }
}
