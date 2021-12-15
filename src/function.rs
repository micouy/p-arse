//! Implementation of [`Parser`] for functions.

use std::marker::PhantomData;

use crate::{Parser, Result};

pub trait Fun<T>: for<'a> Fn(&'a str) -> Result<'a, T> {}
impl<F, T> Fun<T> for F where F: for<'a> Fn(&'a str) -> Result<'a, T> {}

pub trait Rec<T>:
    for<'a> Fn(&'a str, &RecursiveFunction<'_, T>) -> Result<'a, T>
{
}
impl<F, T> Rec<T> for F where
    F: for<'a> Fn(&'a str, &RecursiveFunction<'_, T>) -> Result<'a, T>
{
}

pub struct RecursiveFunction<'f, T> {
    // f: &'f dyn for<'a, 'b> Fn(&'a str, &RecursiveFunction<'b, T>) ->
    // Result<'a, T>,
    f: &'f dyn Rec<T>,
    marker: PhantomData<T>,
}

impl<'f, T> Clone for RecursiveFunction<'f, T> {
    fn clone(&self) -> Self {
        Self {
            f: self.f.clone(),
            marker: PhantomData,
        }
    }
}

pub struct Function<'f, T> {
    // f: &'f dyn for<'a> Fn(&'a str) -> Result<'a, T>,
    f: &'f dyn Fun<T>,
    marker: PhantomData<T>,
}

impl<'f, T> Clone for Function<'f, T> {
    fn clone(&self) -> Self {
        Self {
            f: self.f.clone(),
            marker: PhantomData,
        }
    }
}

impl<'f, 'a, T> Parser<'a> for RecursiveFunction<'f, T> {
    type Output = T;

    fn p_arse(&self, tail: &'a str) -> Result<'a, T> {
        (self.f)(tail, &self)
    }
}

impl<'f, 'a, T> Parser<'a> for Function<'f, T> {
    type Output = T;

    fn p_arse(&self, tail: &'a str) -> Result<'a, T> {
        (self.f)(tail)
    }
}

// pub fn fun<'f, T>(f: &'f dyn for<'a> Fn(&'a str) -> Result<'a, T>) ->
// Function<'f, T>  {
pub fn fun<'f, T>(f: &'f dyn Fun<T>) -> Function<'f, T> {
    Function {
        f,
        marker: PhantomData,
    }
}

// pub fn rec<'f, T>(f: &'f dyn for<'a, 'b> Fn(&'a str, &RecursiveFunction<'b,
// T>) -> Result<'a, T>) -> RecursiveFunction<'f, T> {
pub fn rec<'f, T>(f: &'f dyn Rec<T>) -> RecursiveFunction<'f, T> {
    RecursiveFunction {
        f,
        marker: PhantomData,
    }
}
