//! Error.

/// Main error.
///
/// For now it's only a dummy value.
#[derive(Debug)]
pub struct Error {
    pub stack: Vec<String>,
    pub expectation: String,
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

    pub(crate) fn push(mut self, name: String) -> Self {
        self.stack.push(name);

        self
    }
}

pub type Result<'a, T> = std::result::Result<(T, &'a str), Error>;
