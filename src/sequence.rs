#![allow(clippy::type_complexity)]

//! Sequences of up to 6 elements.

use crate::{parser::Parser, Result};

impl<'a, P0, E> Parser<'a, E> for (P0,)
where
    P0: Parser<'a, E>,
{
    type Output = (P0::Output,);

    fn try_p_arse(&self, tail: &'a str) -> Result<'a, Self::Output, E> {
        let (output_0, tail) = self.0.try_p_arse(tail)?;

        Ok(((output_0,), tail))
    }
}

impl<'a, P0, P1, E> Parser<'a, E> for (P0, P1)
where
    P0: Parser<'a, E>,
    P1: Parser<'a, E>,
{
    type Output = (P0::Output, P1::Output);

    fn try_p_arse(&self, tail: &'a str) -> Result<'a, Self::Output, E> {
        let (output_0, tail) = self.0.try_p_arse(tail)?;
        let (output_1, tail) = self.1.try_p_arse(tail)?;

        Ok(((output_0, output_1), tail))
    }
}

impl<'a, P0, P1, P2, E> Parser<'a, E> for (P0, P1, P2)
where
    P0: Parser<'a, E>,
    P1: Parser<'a, E>,
    P2: Parser<'a, E>,
{
    type Output = (P0::Output, P1::Output, P2::Output);

    fn try_p_arse(&self, tail: &'a str) -> Result<'a, Self::Output, E> {
        let (output_0, tail) = self.0.try_p_arse(tail)?;
        let (output_1, tail) = self.1.try_p_arse(tail)?;
        let (output_2, tail) = self.2.try_p_arse(tail)?;

        Ok(((output_0, output_1, output_2), tail))
    }
}

impl<'a, P0, P1, P2, P3, E> Parser<'a, E> for (P0, P1, P2, P3)
where
    P0: Parser<'a, E>,
    P1: Parser<'a, E>,
    P2: Parser<'a, E>,
    P3: Parser<'a, E>,
{
    type Output = (P0::Output, P1::Output, P2::Output, P3::Output);

    fn try_p_arse(&self, tail: &'a str) -> Result<'a, Self::Output, E> {
        let (output_0, tail) = self.0.try_p_arse(tail)?;
        let (output_1, tail) = self.1.try_p_arse(tail)?;
        let (output_2, tail) = self.2.try_p_arse(tail)?;
        let (output_3, tail) = self.3.try_p_arse(tail)?;

        Ok(((output_0, output_1, output_2, output_3), tail))
    }
}

impl<'a, P0, P1, P2, P3, P4, E> Parser<'a, E> for (P0, P1, P2, P3, P4)
where
    P0: Parser<'a, E>,
    P1: Parser<'a, E>,
    P2: Parser<'a, E>,
    P3: Parser<'a, E>,
    P4: Parser<'a, E>,
{
    type Output = (P0::Output, P1::Output, P2::Output, P3::Output, P4::Output);

    fn try_p_arse(&self, tail: &'a str) -> Result<'a, Self::Output, E> {
        let (output_0, tail) = self.0.try_p_arse(tail)?;
        let (output_1, tail) = self.1.try_p_arse(tail)?;
        let (output_2, tail) = self.2.try_p_arse(tail)?;
        let (output_3, tail) = self.3.try_p_arse(tail)?;
        let (output_4, tail) = self.4.try_p_arse(tail)?;

        Ok(((output_0, output_1, output_2, output_3, output_4), tail))
    }
}

impl<'a, P0, P1, P2, P3, P4, P5, E> Parser<'a, E> for (P0, P1, P2, P3, P4, P5)
where
    P0: Parser<'a, E>,
    P1: Parser<'a, E>,
    P2: Parser<'a, E>,
    P3: Parser<'a, E>,
    P4: Parser<'a, E>,
    P5: Parser<'a, E>,
{
    type Output = (
        P0::Output,
        P1::Output,
        P2::Output,
        P3::Output,
        P4::Output,
        P5::Output,
    );

    fn try_p_arse(&self, tail: &'a str) -> Result<'a, Self::Output, E> {
        let (output_0, tail) = self.0.try_p_arse(tail)?;
        let (output_1, tail) = self.1.try_p_arse(tail)?;
        let (output_2, tail) = self.2.try_p_arse(tail)?;
        let (output_3, tail) = self.3.try_p_arse(tail)?;
        let (output_4, tail) = self.4.try_p_arse(tail)?;
        let (output_5, tail) = self.5.try_p_arse(tail)?;

        Ok((
            (output_0, output_1, output_2, output_3, output_4, output_5),
            tail,
        ))
    }
}
