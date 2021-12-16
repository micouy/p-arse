//! Error.

use std::convert::Infallible;

/// Main error.
#[derive(Debug)]
pub struct Error<E = Infallible> {
    pub stack: Vec<&'static str>,
    kind: ErrorKind<E>,
}

#[derive(Debug)]
enum ErrorKind<E> {
    UnexpectedInput(String),
    User(E),
}

impl<E> Error<E> {
    pub(crate) fn expecting<S>(expectation: S) -> Self
    where
        S: Into<String>,
    {
        Error {
            stack: vec![],
            kind: ErrorKind::UnexpectedInput(expectation.into()),
        }
    }

    pub(crate) fn user(error: E) -> Self {
        Error {
            stack: vec![],
            kind: ErrorKind::User(error),
        }
    }

    pub(crate) fn push(mut self, name: &'static str) -> Self {
        self.stack.push(name);

        self
    }
}

pub type Result<'a, T, E = Infallible> =
    std::result::Result<(T, &'a str), Error<E>>;
