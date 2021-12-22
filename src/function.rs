//! Implementation of [`Parser`] for functions.

use crate::{Parser, Result};

pub trait Fun<T>: for<'a> Fn(&'a str) -> Result<'a, T> {}

impl<F, T> Fun<T> for F where F: for<'a> Fn(&'a str) -> Result<'a, T> {}
pub struct Function<'f, T> {
    f: &'f dyn Fun<T>,
}

impl<'f, T> Clone for Function<'f, T> {
    fn clone(&self) -> Self {
        Self { f: self.f }
    }
}

impl<'f, T> Copy for Function<'f, T> {}

impl<'f, T> Parser for Function<'f, T> {
    type Output = T;

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
        (self.f)(tail)
    }
}

pub fn fun<T>(f: &dyn Fun<T>) -> Function<'_, T> {
    Function { f }
}

pub trait Rec<T>:
    for<'a> Fn(&'a str, RecursiveFunction<'_, T>) -> Result<'a, T>
{
}

impl<F, T> Rec<T> for F where
    F: for<'a> Fn(&'a str, RecursiveFunction<'_, T>) -> Result<'a, T>
{
}

pub struct RecursiveFunction<'f, T> {
    f: &'f dyn Rec<T>,
}

impl<'f, T> Clone for RecursiveFunction<'f, T> {
    fn clone(&self) -> Self {
        Self { f: self.f }
    }
}

impl<'f, T> Copy for RecursiveFunction<'f, T> {}

impl<'f, T> Parser for RecursiveFunction<'f, T> {
    type Output = T;

    fn p_arse<'a>(&self, tail: &'a str) -> Result<'a, Self::Output> {
        (self.f)(tail, *self)
    }
}

pub fn rec<'f, T>(f: &'f dyn Rec<T>) -> RecursiveFunction<'f, T> {
    RecursiveFunction { f: f as &dyn Rec<_> }
}
