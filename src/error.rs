//! Error.

/// Main error.
#[derive(Debug)]
pub struct Error<'a> {
    pub stack: Vec<&'static str>,
    expectation: String,
    tail: &'a str,
}

impl<'a> Error<'a> {
    pub(crate) fn expecting<S>(expectation: S, tail: &'a str) -> Self
    where
        S: Into<String>,
    {
        Error {
            stack: vec![],
            expectation: expectation.into(),
            tail,
        }
    }

    pub(crate) fn push(mut self, name: &'static str) -> Self {
        self.stack.push(name);

        self
    }
}

pub type Result<'a, T> = std::result::Result<(T, &'a str), Error<'a>>;
