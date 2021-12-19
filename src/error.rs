//! Error.

/// Main error.
#[derive(Debug)]
pub struct Error {
    pub stack: Vec<&'static str>,
    expectation: String,
}

impl Error {
    pub(crate) fn expecting<S>(expectation: S) -> Self
    where
        S: Into<String>,
    {
        Error {
            stack: vec![],
            expectation: expectation.into(),
        }
    }

    pub(crate) fn push(mut self, name: &'static str) -> Self {
        self.stack.push(name);

        self
    }
}

pub type Result<'a, T> = std::result::Result<(T, &'a str), Error>;
