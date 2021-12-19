#![allow(clippy::type_complexity)]

//! Sequences of up to 6 elements.

use crate::{Parser, Result};

impl<P0> Parser for (P0,)
where
    P0: Parser,
{
    type Output = (P0::Output,);

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
        let (output_0, tail) = self.0.p_arse(tail)?;

        Ok(((output_0,), tail))
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
