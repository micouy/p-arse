use std::{fmt::Debug, marker::PhantomData};

use crate::{parser::Parser, Error, Result};

pub struct ZeroOrMore<P>
where
    P: Parser,
{
    pub(crate) parser: P,
}

impl<P> Clone for ZeroOrMore<P>
where
    P: Parser,
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser,
        }
    }
}

impl<P> Copy for ZeroOrMore<P> where P: Parser {}

impl<P> Parser for ZeroOrMore<P>
where
    P: Parser,
{
    type Output = Vec<P::Output>;

    fn p_arse<'a>(&self, mut tail: &'a str) -> Result<'a, Self::Output> {
        let mut output = vec![];

        while let Ok((output_i, tail_i)) = self.parser.p_arse(tail) {
            tail = tail_i;
            output.push(output_i);
        }

        Ok((output, tail))
    }
}

pub struct OneOrMore<P>
where
    P: Parser,
{
    pub(crate) parser: P,
}

impl<P> Clone for OneOrMore<P>
where
    P: Parser,
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser,
        }
    }
}

impl<P> Copy for OneOrMore<P> where P: Parser {}

impl<P> Parser for OneOrMore<P>
where
    P: Parser,
{
    type Output = Vec<P::Output>;

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
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

pub struct Ignorant<P>
where
    P: Parser,
{
    pub(crate) parser: P,
}

impl<P> Clone for Ignorant<P>
where
    P: Parser,
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser,
        }
    }
}

impl<P> Copy for Ignorant<P> where P: Parser {}

impl<P> Parser for Ignorant<P>
where
    P: Parser,
{
    type Output = ();

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
        self.parser.p_arse(tail).map(|(_, tail)| ((), tail))
    }
}

pub struct Opt<P>
where
    P: Parser,
{
    pub(crate) parser: P,
}

impl<P> Clone for Opt<P>
where
    P: Parser,
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser,
        }
    }
}

impl<P> Copy for Opt<P> where P: Parser {}

impl<P> Parser for Opt<P>
where
    P: Parser,
{
    type Output = Option<P::Output>;

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
        if let Ok((output, tail)) = self.parser.p_arse(tail) {
            Ok((Some(output), tail))
        } else {
            Ok((None, tail))
        }
    }
}

pub struct Map<P, F, U>
where
    P: Parser,
    F: Fn(P::Output) -> U + Copy,
{
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) marker: PhantomData<U>,
}

impl<P, F, U> Clone for Map<P, F, U>
where
    P: Parser,
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

impl<P, F, U> Copy for Map<P, F, U>
where
    P: Parser + Copy,
    F: Fn(P::Output) -> U + Copy,
{
}

impl<P, F, U> Parser for Map<P, F, U>
where
    F: Fn(P::Output) -> U + Copy,
    P: Parser,
{
    type Output = U;

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
        self.parser
            .p_arse(tail)
            .map(|(t, tail)| ((self.f)(t), tail))
    }
}

pub struct Or<P0, P1>
where
    P0: Parser,
    P1: Parser<Output = P0::Output>,
{
    pub(crate) parser_0: P0,
    pub(crate) parser_1: P1,
}

impl<P0, P1> Clone for Or<P0, P1>
where
    P0: Parser,
    P1: Parser<Output = P0::Output>,
{
    fn clone(&self) -> Self {
        Self {
            parser_0: self.parser_0,
            parser_1: self.parser_1,
        }
    }
}

impl<P0, P1> Copy for Or<P0, P1>
where
    P0: Parser,
    P1: Parser<Output = P0::Output>,
{
}

impl<P0, P1> Parser for Or<P0, P1>
where
    P0: Parser,
    P1: Parser<Output = P0::Output>,
{
    type Output = P0::Output;

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
        if let Ok((output, tail)) = self.parser_0.p_arse(tail) {
            Ok((output, tail))
        } else {
            self.parser_1.p_arse(tail)
        }
    }
}

pub struct NegativeLookahead<P>
where
    P: Parser,
{
    pub(crate) parser: P,
}

impl<P> Clone for NegativeLookahead<P>
where
    P: Parser,
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser,
        }
    }
}

impl<P> Copy for NegativeLookahead<P> where P: Parser {}

impl<P> Parser for NegativeLookahead<P>
where
    P: Parser,
{
    type Output = ();

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
        if self.parser.p_arse(tail).is_err() {
            Ok(((), tail))
        } else {
            // TODO what to put here?
            Err(Error::expecting("negative lookahead", tail))
        }
    }
}

pub struct PositiveLookahead<P>
where
    P: Parser,
{
    pub(crate) parser: P,
}

impl<P> Clone for PositiveLookahead<P>
where
    P: Parser,
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser,
        }
    }
}

impl<P> Copy for PositiveLookahead<P> where P: Parser {}

impl<P> Parser for PositiveLookahead<P>
where
    P: Parser,
{
    type Output = ();

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
        if self.parser.p_arse(tail).is_ok() {
            Ok(((), tail))
        } else {
            // TODO what to put here?
            Err(Error::expecting("positive lookahead", tail))
        }
    }
}

pub struct Named<P>
where
    P: Parser,
{
    pub(crate) parser: P,
    pub(crate) name: &'static str,
}

impl<P> Clone for Named<P>
where
    P: Parser,
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser,
            name: self.name,
        }
    }
}

impl<P> Copy for Named<P> where P: Parser {}

impl<P> Parser for Named<P>
where
    P: Parser,
{
    type Output = P::Output;

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
        self.parser.p_arse(tail).map_err(|err| err.push(self.name))
    }
}

pub struct MapStr<P, F, T>
where
    P: Parser,
    F: Fn(&str) -> T + Copy,
{
    pub(crate) parser: P,
    pub(crate) f: F,
}

impl<P, F, T> Clone for MapStr<P, F, T>
where
    P: Parser,
    F: Fn(&str) -> T + Copy,
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser,
            f: self.f,
        }
    }
}

impl<P, F, T> Copy for MapStr<P, F, T>
where
    P: Parser,
    F: Fn(&str) -> T + Copy,
{
}


impl<P, F, T> Parser for MapStr<P, F, T>
where
    P: Parser,
    F: Fn(&str) -> T + Copy,
{
    type Output = T;

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
        match self.parser.p_arse(tail) {
            Ok((_, new_tail)) => {
                let len_diff = tail.len() - new_tail.len();
                let captured = &tail[0..len_diff];
                let value = (self.f)(captured);

                Ok((value, new_tail))
            }
            Err(err) => Err(err),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Debugged<P>
where
    P: Parser,
    P::Output: Debug,
{
    pub(crate) parser: P,
}

impl<P> Parser for Debugged<P>
where
    P: Parser,
    P::Output: Debug,
{
    type Output = P::Output;

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
        dbg!(self.parser.p_arse(tail))
    }
}
