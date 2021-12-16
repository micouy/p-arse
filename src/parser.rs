//! The core functionality.

use std::{convert::Infallible, marker::PhantomData};

use crate::{wrapper::*, Result};

/// The main trait.
///
/// A [`Parser`] may be:
/// - a [`char`], a [`CharRange`](crate::literal::CharRange) constructed using
///   [`'a'.to('z')`](crate::CharExt::to), or a [`&str`], all corresponding to
///   terminals (or sequences of terminals) in PEG,
/// - a (possibly recurisve) function from [`&str`] to
///   [`p_arse::Result`](crate::Result), corresponding to non-terminals in PEG,
/// - a tuple of up to 6 [`Parser`]s, corresponding to a sequence in PEG,
/// - any parser constructed using one of the [`Parser`]'s methods,
///   corresponding to various operators in PEG,
/// - [`any()`](crate::any), matching any character,
/// - [`eoi()`](crate::eoi), matching the end of input.
///
/// # Notes on designing new higher order parsers
///
/// The set of higher order parsers (constructed using [`Parser`]'s methods)
/// is not meant to be extended by the user. The provided methods aim to be
/// general enough to suffice in most cases. However, if the user wishes
/// to do it anyways, it is advised to define a new `ParserExt<'a>` trait and
/// implement it for all `P: Parser<'a>`. The choice of using methods over
/// functions in `p-arse` was deliberate and this solution ensures the
/// consistency of syntax and readability.
///
/// ```
/// use p_arse::Parser;
///
/// use std::marker::PhantomData;
///
/// struct MyParser<'a, P> where P: Parser<'a> {
///     parser: P,
///     marker: PhantomData<&'a ()>,
/// }
///
/// trait ParserExt<'a>: Parser<'a> {
///     fn my_parser(self) -> MyParser<'a, Self> {
///         MyParser { parser: self, marker: PhantomData }
///     }
/// }
///
/// impl<'a, P> ParserExt<'a> for P where P: Parser<'a> {}
///
/// fn main() {
///     let higher_order_parser = 'a'.my_parser();
/// }
/// ```
pub trait Parser<'a, E = Infallible>: Sized + Copy {
    type Output;

    /// Attempts to parse the input.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_arse::{Parser, any};
    ///
    /// let just_a = 'a';
    /// let result = just_a.p_arse("abc");
    /// ```
    fn p_arse(
        &self,
        tail: &'a str,
    ) -> Result<'a, <Self as Parser<'a, E>>::Output, Infallible>
    where
        Self: Parser<'a, Infallible, Output = <Self as Parser<'a, E>>::Output>,
    {
        self.try_p_arse(tail)
    }

    fn try_p_arse(&self, tail: &'a str) -> Result<'a, Self::Output, E>;

    /// Maps the parser's output.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_arse::{Parser, any};
    ///
    /// let parse_digit = |d: char| d.to_digit(10).unwrap();
    /// let digit = any().map(parse_digit);
    /// let (digit, _tail) = digit.p_arse("1").unwrap();
    ///
    /// assert_eq!(digit, 1);
    /// ```
    fn map<F, U>(self, f: F) -> Map<'a, Self, F, U, E>
    where
        F: Fn(Self::Output) -> U + Copy,
    {
        Map {
            parser: self,
            f,
            marker: PhantomData,
        }
    }

    fn and<F, U>(self, f: F) -> AndThen<'a, Self, F, U, E>
    where
        F: Fn(Self::Output) -> std::result::Result<U, E> + Copy,
    {
        AndThen {
            parser: self,
            f,
            marker: PhantomData,
        }
    }

    /// Ignores the parser's output and returns `()` instead.
    ///
    /// The effect is the same as `.map(|_| ())`. It's useful when dealing with
    /// the type signature of a parser function.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_arse::{Parser, Result, rec, function::RecursiveFunction};
    ///
    /// // Without `.ignore()` the function would return a cyclic type of infinite size.
    /// let a_string = rec(&|tail, a_string| {
    ///     ("a", a_string.opt()).ignore().p_arse(tail)
    /// });
    /// ```
    fn ignore(self) -> Ignorant<'a, Self, E> {
        Ignorant {
            parser: self,
            marker: PhantomData,
        }
    }

    /// Returns an alternative of the two parsers.
    ///
    /// The alternative is short-circuiting and returns the result of the first
    /// successful parsing. If neither of the parsers match the input, it
    /// returns an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_arse::Parser;
    ///
    /// let a_or_b = 'a'.or('b');
    ///
    /// assert!(a_or_b.p_arse("a").is_ok());
    /// assert!(a_or_b.p_arse("b").is_ok());
    /// assert!(a_or_b.p_arse("c").is_err());
    /// ```
    ///
    /// `.or()` can be chained.
    ///
    /// ```
    /// # use p_arse::Parser;
    /// let a_or_b_or_c = 'a'.or('b').or('c');
    /// ```
    fn or<P>(self, other: P) -> Or<'a, Self, P, E>
    where
        P: Parser<'a, E, Output = Self::Output>,
    {
        Or {
            parser_0: self,
            parser_1: other,
            marker: PhantomData,
        }
    }

    /// Makes the parser optional.
    ///
    /// The returned higher order parser always succeeds and returns
    /// [`Some`](Option::Some) containing the output from parsing if it
    /// succeeded or [`None`] if it didn't.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_arse::Parser;
    ///
    /// let to_be_or_not_to_be = "to be".opt();
    ///
    /// assert!(to_be_or_not_to_be.p_arse("to be").is_ok());
    /// assert!(to_be_or_not_to_be.p_arse("definitely not 'to be'").is_ok());
    /// ```
    fn opt(self) -> Opt<'a, Self, E> {
        Opt {
            parser: self,
            marker: PhantomData,
        }
    }

    /// Makes the parser match **z**ero or m**ore** times.
    ///
    /// The returned higher order parser always succeeds and returns a [`Vec`]
    /// containing the outputs from the successful parsings.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_arse::{Parser, any};
    ///
    /// let anything = any().zore();
    ///
    /// assert!(anything.p_arse("").is_ok());
    /// assert!(anything.p_arse("a").is_ok());
    /// assert!(anything.p_arse("ab").is_ok());
    /// assert!(anything.p_arse("abc").is_ok());
    /// ```
    fn zore(self) -> ZeroOrMore<'a, Self, E> {
        ZeroOrMore {
            parser: self,
            marker: PhantomData,
        }
    }

    /// Makes the parser match one or more times.
    ///
    /// The returned higher order parser returns a [`Vec`] containing the
    /// outputs from the successful parsings.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_arse::Parser;
    ///
    /// let bees = 'b'.more();
    ///
    /// assert!(bees.p_arse("").is_err()); // Must begin with at least one 'b'!
    /// assert!(bees.p_arse("b").is_ok());
    /// assert!(bees.p_arse("bb").is_ok());
    /// assert!(bees.p_arse("bbb").is_ok());
    /// ```
    fn more(self) -> OneOrMore<'a, Self, E> {
        OneOrMore {
            parser: self,
            marker: PhantomData,
        }
    }

    /// Turns the parser into a negative look-ahead.
    ///
    /// A look-ahead never consumes its input and may only return `()`. The
    /// returned higher order parser succeeds iff the supplied parser
    /// didn't.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_arse::Parser;
    ///
    /// let does_not_begin_with_a = 'a'.not_ahead();
    ///
    /// assert!(does_not_begin_with_a.p_arse("bxxx").is_ok());
    /// assert!(does_not_begin_with_a.p_arse("axxx").is_err());
    /// ```
    ///
    /// The input is not consumed.
    ///
    /// ```
    /// use p_arse::Parser;
    ///
    /// let does_not_consume = 'x'.not_ahead();
    /// let ((), tail) = does_not_consume.p_arse("abc").unwrap();
    ///
    /// assert_eq!(tail, "abc");
    /// ```
    fn not_ahead(self) -> NegativeLookahead<'a, Self, E> {
        NegativeLookahead {
            parser: self,
            marker: PhantomData,
        }
    }

    /// Turns the parser into a positive look-ahead.
    ///
    /// A look-ahead never consumes its input and may only return `()`. The
    /// returned higher order parser succeeds iff the supplied parser did.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_arse::Parser;
    ///
    /// let begins_with_a = 'a'.ahead();
    ///
    /// assert!(begins_with_a.p_arse("axxx").is_ok());
    /// assert!(begins_with_a.p_arse("bxxx").is_err());
    /// ```
    ///
    /// The input is not consumed.
    ///
    /// ```
    /// use p_arse::Parser;
    ///
    /// let does_not_consume = 'a'.ahead();
    /// let ((), tail) = does_not_consume.p_arse("abc").unwrap();
    ///
    /// assert_eq!(tail, "abc");
    /// ```
    fn ahead(self) -> PositiveLookahead<'a, Self, E> {
        PositiveLookahead {
            parser: self,
            marker: PhantomData,
        }
    }

    fn named(self, name: &'static str) -> Named<'a, Self, E> {
        Named {
            parser: self,
            marker: PhantomData,
            name,
        }
    }
}
