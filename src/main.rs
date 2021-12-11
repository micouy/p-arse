#![feature(pattern, fn_traits, str_split_inclusive_as_str)]

use std::{marker::PhantomData, str::pattern};

#[derive(Debug)]
struct Error {}

type Result<'a, T> = std::result::Result<(T, &'a str), Error>;

trait Parser<'a>: Sized {
    type Output;

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output>;

    fn map<F, U>(self, f: F) -> Map<'a, Self, F, U>
    where
        F: Fn(Self::Output) -> U,
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

// impl Parser for F (and &F and &&F etc...)
impl<'a, F, T> Parser<'a> for F
where
    F: Fn(&'a str) -> Result<'a, T>,
{
    type Output = T;

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        self(tail)
    }
}

// impl Parser for &P and &&P etc... if P: Parser and P != F (how?)
// impl<'a, P> Parser<'a> for &P where P: Parser<'a> {
// type Output = P::Output;
//
// fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
// self.p_arse(tail)
// }
// }

struct ZeroOrMore<'a, P>
where
    P: Parser<'a>,
{
    parser: P,
    marker: PhantomData<&'a ()>,
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

struct OneOrMore<'a, P>
where
    P: Parser<'a>,
{
    parser: P,
    marker: PhantomData<&'a ()>,
}

impl<'a, P> Parser<'a> for OneOrMore<'a, P>
where
    P: Parser<'a>,
{
    type Output = Vec<P::Output>;

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        let (first, mut tail) = self.parser.p_arse(tail)?;
        let mut output = vec![first];

        while let Ok((output_i, tail_i)) = self.parser.p_arse(tail) {
            tail = tail_i;
            output.push(output_i);
        }

        Ok((output, tail))
    }
}

struct Ignorant<'a, P>
where
    P: Parser<'a>,
{
    parser: P,
    marker: PhantomData<&'a ()>,
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

struct Pattern<'a, P>
where
    P: pattern::Pattern<'a> + Copy,
{
    pattern: P,
    marker: PhantomData<&'a ()>,
}

impl<'a, P> Parser<'a> for Pattern<'a, P>
where
    P: pattern::Pattern<'a> + Copy,
{
    type Output = &'a str;

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        let stripped = tail.strip_prefix(self.pattern).ok_or(Error {})?;
        let len_diff = tail.len() - stripped.len();
        let head = &tail[0..len_diff];

        Ok((head, stripped))
    }
}

fn pat<'a, P>(pattern: P) -> Pattern<'a, P>
where
    P: pattern::Pattern<'a> + Copy,
{
    Pattern {
        pattern,
        marker: PhantomData,
    }
}

struct Opt<'a, P>
where
    P: Parser<'a>,
{
    parser: P,
    marker: PhantomData<&'a ()>,
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

struct Map<'a, P, F, U>
where
    P: Parser<'a>,
    F: Fn(P::Output) -> U,
{
    parser: P,
    f: F,
    lifetime_marker: PhantomData<&'a ()>,
    u_marker: PhantomData<U>,
}

impl<'a, P, F, U> Parser<'a> for Map<'a, P, F, U>
where
    F: Fn(P::Output) -> U,
    P: Parser<'a>,
{
    type Output = U;

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        self.parser
            .p_arse(tail)
            .map(|(t, tail)| ((self.f)(t), tail))
    }
}

impl<'a, P0> Parser<'a> for (P0,)
where
    P0: Parser<'a>,
{
    type Output = (P0::Output,);

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        let (output_0, tail) = self.0.p_arse(tail)?;

        Ok(((output_0,), tail))
    }
}

impl<'a, P0, P1> Parser<'a> for (P0, P1)
where
    P0: Parser<'a>,
    P1: Parser<'a>,
{
    type Output = (P0::Output, P1::Output);

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        let (output_0, tail) = self.0.p_arse(tail)?;
        let (output_1, tail) = self.1.p_arse(tail)?;

        Ok(((output_0, output_1), tail))
    }
}

impl<'a, P0, P1, P2> Parser<'a> for (P0, P1, P2)
where
    P0: Parser<'a>,
    P1: Parser<'a>,
    P2: Parser<'a>,
{
    type Output = (P0::Output, P1::Output, P2::Output);

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        let (output_0, tail) = self.0.p_arse(tail)?;
        let (output_1, tail) = self.1.p_arse(tail)?;
        let (output_2, tail) = self.2.p_arse(tail)?;

        Ok(((output_0, output_1, output_2), tail))
    }
}

struct Or<'a, P0, P1>
where
    P0: Parser<'a>,
    P1: Parser<'a, Output = P0::Output>,
{
    parser_0: P0,
    parser_1: P1,
    marker: PhantomData<&'a ()>,
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

struct NegativePredicate<'a, P> where P: Parser<'a> {
    parser: P,
    marker: PhantomData<&'a ()>,
}

impl<'a, P> Parser<'a> for NegativePredicate<'a, P> where P: Parser<'a> {
    type Output = ();

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
		if self.parser.p_arse(tail).is_err() { 
    		Ok(((), tail))
		} else {
			Err(Error {})
		}
	}
}

struct PositivePredicate<'a, P> where P: Parser<'a> {
    parser: P,
    marker: PhantomData<&'a ()>,
}

impl<'a, P> Parser<'a> for PositivePredicate<'a, P> where P: Parser<'a> {
    type Output = ();

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
		if self.parser.p_arse(tail).is_ok() { 
    		Ok(((), tail))
		} else {
			Err(Error {})
		}
	}
}

struct Any {}

impl<'a> Parser<'a> for Any {
    type Output = char;

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        let mut chars = tail.chars();
        let first = chars.next().ok_or(Error {})?;
        let tail = chars.as_str();

        Ok((first, tail))
    }
}

fn any() -> Any {
    Any {}
}

impl<'a> Parser<'a> for &str {
    type Output = &'a str;

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        pat(self).p_arse(tail)
    }
}

impl<'a> Parser<'a> for char {
    type Output = &'a str;

    fn p_arse(&self, tail: &'a str) -> Result<'a, Self::Output> {
        pat(*self).p_arse(tail)
    }
}

fn main() {
    // String slices.
    let just_a = "a"; // "a"
    assert!(just_a.p_arse("a").is_ok());
    assert!(just_a.p_arse("b").is_err());


    // Patterns.
    let ascii = pat(|c: char| c.is_ascii());
    assert!(ascii.p_arse("abcde").is_ok());
    assert!(ascii.p_arse("łajza").is_err());


    // Sequences.
    let abc = ("a", "b", "c"); // "a" "b" "c"
    assert!(abc.p_arse("abc").is_ok());
    assert!(abc.p_arse("xxx").is_err());


    // Or.
    let a_or_b_or_c = "a".or("b").or("c"); // "a" / "b" / "c"
    assert!(a_or_b_or_c.p_arse("axxx").is_ok());
    assert!(a_or_b_or_c.p_arse("bxxx").is_ok());
    assert!(a_or_b_or_c.p_arse("cxxx").is_ok());


    // Mapping.
    let parse_digit = |d: char| d.to_digit(10).unwrap();
    let digit = any().map(parse_digit);
    assert_eq!(digit.p_arse("1").unwrap().0, 1);


    // Recursive parsers.

    // A = "a" A?
    fn a_string<'a>(tail: &'a str) -> Result<'a, ()> {
        ("a", a_string.opt()).ignore().p_arse(tail)
    }

    assert!(a_string.p_arse("").is_err());
    assert!(a_string.p_arse("a").is_ok());
    assert!(a_string.p_arse("aa").is_ok());
    assert!(a_string.p_arse("aaa").is_ok());


    // Repetition.
    let bees = "b".zore(); // "b"*
    assert!(bees.p_arse("").is_ok());
    assert!(bees.p_arse("b").is_ok());
    assert!(bees.p_arse("bb").is_ok());
    assert!(bees.p_arse("bbb").is_ok());

    let scream = "a".more(); // "a"+
    assert!(scream.p_arse("").is_err());
    assert!(scream.p_arse("a").is_ok());
    assert!(scream.p_arse("aa").is_ok());
    assert!(scream.p_arse("aaa").is_ok());


    // Look-ahead.
    let a_ahead = "a".ahead();
    assert!(a_ahead.p_arse("aaa").is_ok());
    assert!(a_ahead.p_arse("bbb").is_err());

    let a_not_ahead = "a".not_ahead();
    assert!(a_not_ahead.p_arse("bbb").is_ok());
    assert!(a_not_ahead.p_arse("aaa").is_err());


    // All at once.
    let abc = ("a", "b", "c"); // "a" "b" "c"
    let join_abc =
        |(a, b, c): (&str, &str, &str)| -> String { a.to_owned() + b + c };
    let abc = abc.map(join_abc);

    let xd = ("x", "d".opt()); // "x" "d"?
    let join_xd = |(x, d): (&str, Option<&str>)| -> String {
        x.to_owned() + d.unwrap_or("")
    };
    let xd = xd.map(join_xd);

    let abcxd = abc.or(xd);

    assert_eq!(abcxd.p_arse("abc").unwrap().0, "abc");
    assert_eq!(abcxd.p_arse("xd").unwrap().0, "xd");
    assert_eq!(abcxd.p_arse("x").unwrap().0, "x");

	let until_newline = ('\n'.not_ahead(), any()).zore();
	dbg!(until_newline.p_arse("first line\nsecond line\n").unwrap().1);
}
