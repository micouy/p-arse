#![allow(clippy::type_complexity)]

//! Sequences of up to 6 elements.

use duple::prelude::*;

use crate::{Parser, Result};

impl<P0> Parser for (P0,)
where
    P0: Parser,
{
    type Output = P0::Output;

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
        let (output_0, tail) = self.0.p_arse(tail)?;

        Ok((output_0, tail))
    }
}

impl<P0, P1> Parser for (P0, P1)
where
    P0: Parser,
    P1: Parser,
{
    type Output = (P0::Output, P1::Output);

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
        let (output_0, tail) = self.0.p_arse(tail)?;
        let (output_1, tail) = self.1.p_arse(tail)?;

        Ok(((output_0, output_1), tail))
    }
}

impl<P0, P1, P2> Parser for (P0, P1, P2)
where
    P0: Parser,
    P1: Parser,
    P2: Parser,
{
    type Output = (P0::Output, P1::Output, P2::Output);

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
        let (output_0, tail) = self.0.p_arse(tail)?;
        let (output_1, tail) = self.1.p_arse(tail)?;
        let (output_2, tail) = self.2.p_arse(tail)?;

        Ok(((output_0, output_1, output_2), tail))
    }
}

impl<P0, P1, P2, P3> Parser for (P0, P1, P2, P3)
where
    P0: Parser,
    P1: Parser,
    P2: Parser,
    P3: Parser,
{
    type Output = (P0::Output, P1::Output, P2::Output, P3::Output);

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
        let (output_0, tail) = self.0.p_arse(tail)?;
        let (output_1, tail) = self.1.p_arse(tail)?;
        let (output_2, tail) = self.2.p_arse(tail)?;
        let (output_3, tail) = self.3.p_arse(tail)?;

        Ok(((output_0, output_1, output_2, output_3), tail))
    }
}

impl<P0, P1, P2, P3, P4> Parser for (P0, P1, P2, P3, P4)
where
    P0: Parser,
    P1: Parser,
    P2: Parser,
    P3: Parser,
    P4: Parser,
{
    type Output = (P0::Output, P1::Output, P2::Output, P3::Output, P4::Output);

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
        let (output_0, tail) = self.0.p_arse(tail)?;
        let (output_1, tail) = self.1.p_arse(tail)?;
        let (output_2, tail) = self.2.p_arse(tail)?;
        let (output_3, tail) = self.3.p_arse(tail)?;
        let (output_4, tail) = self.4.p_arse(tail)?;

        Ok(((output_0, output_1, output_2, output_3, output_4), tail))
    }
}

impl<P0, P1, P2, P3, P4, P5> Parser for (P0, P1, P2, P3, P4, P5)
where
    P0: Parser,
    P1: Parser,
    P2: Parser,
    P3: Parser,
    P4: Parser,
    P5: Parser,
{
    type Output = (
        P0::Output,
        P1::Output,
        P2::Output,
        P3::Output,
        P4::Output,
        P5::Output,
    );

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
        let (output_0, tail) = self.0.p_arse(tail)?;
        let (output_1, tail) = self.1.p_arse(tail)?;
        let (output_2, tail) = self.2.p_arse(tail)?;
        let (output_3, tail) = self.3.p_arse(tail)?;
        let (output_4, tail) = self.4.p_arse(tail)?;
        let (output_5, tail) = self.5.p_arse(tail)?;

        Ok((
            (output_0, output_1, output_2, output_3, output_4, output_5),
            tail,
        ))
    }
}

pub trait TupleExt: Parser {
    fn rem0(self) -> Remove0<Self>
    where
        Self::Output: TupleRemove0,
    {
        Remove0 { parser: self }
    }

    fn rem1(self) -> Remove1<Self>
    where
        Self::Output: TupleRemove1,
    {
        Remove1 { parser: self }
    }

    fn rem2(self) -> Remove2<Self>
    where
        Self::Output: TupleRemove2,
    {
        Remove2 { parser: self }
    }

    fn rem3(self) -> Remove3<Self>
    where
        Self::Output: TupleRemove3,
    {
        Remove3 { parser: self }
    }

    fn rem4(self) -> Remove4<Self>
    where
        Self::Output: TupleRemove4,
    {
        Remove4 { parser: self }
    }

    fn rem5(self) -> Remove5<Self>
    where
        Self::Output: TupleRemove5,
    {
        Remove5 { parser: self }
    }
}

impl<P> TupleExt for P where P: Parser {}

#[derive(Copy, Clone)]
pub struct Remove0<P>
where
    P: Parser,
    P::Output: TupleRemove0,
{
    parser: P,
}

impl<P> Parser for Remove0<P>
where
    P: Parser,
    P::Output: TupleRemove0,
{
    type Output = <<P as Parser>::Output as TupleRemove0>::Removed;

    fn p_arse<'a>(&self, input: &'a str) -> Result<'a, Self::Output> {
        self.parser
            .p_arse(input)
            .map(|(tuple, tail)| (tuple.rem0(), tail))
    }
}

#[derive(Copy, Clone)]
pub struct Remove1<P>
where
    P: Parser,
    P::Output: TupleRemove1,
{
    parser: P,
}

impl<P> Parser for Remove1<P>
where
    P: Parser,
    P::Output: TupleRemove1,
{
    type Output = <<P as Parser>::Output as TupleRemove1>::Removed;

    fn p_arse<'a>(&self, input: &'a str) -> Result<'a, Self::Output> {
        self.parser
            .p_arse(input)
            .map(|(tuple, tail)| (tuple.rem1(), tail))
    }
}

#[derive(Copy, Clone)]
pub struct Remove2<P>
where
    P: Parser,
    P::Output: TupleRemove2,
{
    parser: P,
}

impl<P> Parser for Remove2<P>
where
    P: Parser,
    P::Output: TupleRemove2,
{
    type Output = <<P as Parser>::Output as TupleRemove2>::Removed;

    fn p_arse<'a>(&self, input: &'a str) -> Result<'a, Self::Output> {
        self.parser
            .p_arse(input)
            .map(|(tuple, tail)| (tuple.rem2(), tail))
    }
}

#[derive(Copy, Clone)]
pub struct Remove3<P>
where
    P: Parser,
    P::Output: TupleRemove3,
{
    parser: P,
}

impl<P> Parser for Remove3<P>
where
    P: Parser,
    P::Output: TupleRemove3,
{
    type Output = <<P as Parser>::Output as TupleRemove3>::Removed;

    fn p_arse<'a>(&self, input: &'a str) -> Result<'a, Self::Output> {
        self.parser
            .p_arse(input)
            .map(|(tuple, tail)| (tuple.rem3(), tail))
    }
}

#[derive(Copy, Clone)]
pub struct Remove4<P>
where
    P: Parser,
    P::Output: TupleRemove4,
{
    parser: P,
}

impl<P> Parser for Remove4<P>
where
    P: Parser,
    P::Output: TupleRemove4,
{
    type Output = <<P as Parser>::Output as TupleRemove4>::Removed;

    fn p_arse<'a>(&self, input: &'a str) -> Result<'a, Self::Output> {
        self.parser
            .p_arse(input)
            .map(|(tuple, tail)| (tuple.rem4(), tail))
    }
}

#[derive(Copy, Clone)]
pub struct Remove5<P>
where
    P: Parser,
    P::Output: TupleRemove5,
{
    parser: P,
}

impl<P> Parser for Remove5<P>
where
    P: Parser,
    P::Output: TupleRemove5,
{
    type Output = <<P as Parser>::Output as TupleRemove5>::Removed;

    fn p_arse<'a>(&self, input: &'a str) -> Result<'a, Self::Output> {
        self.parser
            .p_arse(input)
            .map(|(tuple, tail)| (tuple.rem5(), tail))
    }
}
