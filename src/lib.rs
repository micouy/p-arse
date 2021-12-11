#![feature(pattern, fn_traits, str_split_inclusive_as_str)]

pub mod any;
pub mod char;
pub mod function;
pub mod parser;
pub mod pattern;
pub mod sequence;
pub mod str;
pub mod wrappers;

pub mod prelude {
    pub use crate::{
        any::any,
        char::CharExt,
        parser::{Error, Parser, Result},
    };
}

pub use prelude::*;
