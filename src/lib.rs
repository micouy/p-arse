#![feature(pattern, fn_traits, str_split_inclusive_as_str)]

pub mod function;
pub mod misc;
pub mod parser;
pub mod pattern;
pub mod sequence;
pub mod str;
pub mod wrappers;

pub mod prelude {
    pub use crate::{
        misc::{any, eof},
        parser::{Error, Parser, Result},
        str::CharExt,
    };
}

pub use prelude::*;
