use std::marker::PhantomData;

use crate::wrappers::*;

#[derive(Debug)]
pub struct Error {}

pub type Result<'a, T> = std::result::Result<(T, &'a str), Error>;

pub trait Parser<'a>: Sized + Clone {
    type Output;

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output>;

    fn map<F, U>(self, f: F) -> Map<'a, Self, F, U>
    where
        F: Fn(Self::Output) -> U + Clone,
    {
        Map {
            parser: self,
            f,
            lifetime_marker: PhantomData,
            u_marker: PhantomData,
        }
    }

    fn ignore(self) -> Ignorant<'a, Self> {
        Ignorant {
            parser: self,
            marker: PhantomData,
        }
    }

    fn or<P>(self, other: P) -> Or<'a, Self, P>
    where
        P: Parser<'a, Output = Self::Output>,
    {
        Or {
            parser_0: self,
            parser_1: other,
            marker: PhantomData,
        }
    }

    fn opt(self) -> Opt<'a, Self> {
        Opt {
            parser: self,
            marker: PhantomData,
        }
    }

    fn zore(self) -> ZeroOrMore<'a, Self> {
        ZeroOrMore {
            parser: self,
            marker: PhantomData,
        }
    }

    fn more(self) -> OneOrMore<'a, Self> {
        OneOrMore {
            parser: self,
            marker: PhantomData,
        }
    }

    fn not_ahead(self) -> NegativePredicate<'a, Self> {
        NegativePredicate {
            parser: self,
            marker: PhantomData,
        }
    }

    fn ahead(self) -> PositivePredicate<'a, Self> {
        PositivePredicate {
            parser: self,
            marker: PhantomData,
        }
    }
}
