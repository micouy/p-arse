//! Implementation of [`Parser`] for functions.

use std::{convert::Infallible, marker::PhantomData};

use crate::{Parser, Result};

pub trait Fun<T, E = Infallible>:
    for<'a> Fn(&'a str) -> Result<'a, T, E>
{
}

impl<F, T, E> Fun<T, E> for F where F: for<'a> Fn(&'a str) -> Result<'a, T, E> {}
pub struct Function<'f, T, E = Infallible> {
    f: &'f dyn Fun<T, E>,
    marker: PhantomData<(T, E)>,
}

impl<'f, T, E> Clone for Function<'f, T, E> {
    fn clone(&self) -> Self {
        Self {
            f: self.f,
            marker: PhantomData,
        }
    }
}

impl<'f, T, E> Copy for Function<'f, T, E> {}

impl<'f, 'a, T, E> Parser<'a, E> for Function<'f, T, E> {
    type Output = T;

    fn try_p_arse(&self, tail: &'a str) -> Result<'a, Self::Output, E> {
        (self.f)(tail)
    }
}

pub fn fun<T, E>(f: &dyn Fun<T, E>) -> Function<'_, T, E> {
    Function {
        f,
        marker: PhantomData,
    }
}

pub trait Rec<T, E = Infallible>:
    for<'a> Fn(&'a str, &RecursiveFunction<'_, T, E>) -> Result<'a, T, E>
{
}
impl<F, T, E> Rec<T, E> for F where
    F: for<'a> Fn(&'a str, &RecursiveFunction<'_, T, E>) -> Result<'a, T, E>
{
}

pub struct RecursiveFunction<'f, T, E = Infallible> {
    f: &'f dyn Rec<T, E>,
    marker: PhantomData<(T, E)>,
}

impl<'f, T, E> Clone for RecursiveFunction<'f, T, E> {
    fn clone(&self) -> Self {
        Self {
            f: self.f,
            marker: PhantomData,
        }
    }
}

impl<'f, T, E> Copy for RecursiveFunction<'f, T, E> {}

impl<'f, 'a, T, E> Parser<'a, E> for RecursiveFunction<'f, T, E> {
    type Output = T;

    fn try_p_arse(&self, tail: &'a str) -> Result<'a, Self::Output, E> {
        (self.f)(tail, &self)
    }
}

pub fn rec<T, E>(f: &dyn Rec<T, E>) -> RecursiveFunction<'_, T, E> {
    RecursiveFunction {
        f,
        marker: PhantomData,
    }
}
