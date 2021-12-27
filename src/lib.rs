#![feature(pattern, fn_traits, str_split_inclusive_as_str)]

//! **WARNING**: Thorough documentation is one of the goals. However, at the moment it's not near completion. This is an early version
//! and many things may change in the near future. The documentation will be improved as the crate matures.
//!
//! # `( ㅅ )` the inelegant parser
//!
//! `p-arse` is a PEG parser library. It provides [`Parser`] trait with
//! methods corresponding to PEG parsers and some basic [`utils`].
//!
//! The main focus of this library is simple syntax, type safety and easy
//! debugging. It attempts to follow the original PEG syntax as closely as
//! possible. Speed and efficiency are secondary.
//!
//! For now the library only contains tools for dealing with complete strings.
//! It may be developed in the future to cover byte slices and incomplete
//! input, although I'm not planning to do it at the moment.
//!
//! # Examples
//!
//! [FASTA](https://en.wikipedia.org/wiki/FASTA_format) parser:
//!
//! ```
//! use p_arse::{Parser, CharExt, any, eoi};
//!
//! let nl = '\n';
//!
//! let header_content = (nl.not_ahead(), any()).more();
//! let header_tag = ">";
//! let header = (header_tag, header_content, nl);
//!
//! let sequence_char = ('A'.to('Z')).or('*').or('-');
//! let subsequence = sequence_char.more();
//! let sequence =
//!     (subsequence, (nl, subsequence).zore(), nl.opt());
//!
//! let entry = (header, sequence);
//!
//! let file = (entry.zore(), eoi());
//!
//! # let input = "\
//! # >MCHU - Calmodulin - Human, rabbit, bovine, rat, and chicken\n\
//! # MADQLTEEQIAEFKEAFSLFDKDGDGTITTKELGTVMRSLGQNPTEAELQDMINEVDADGNGTID\n\
//! # FPEFLTMMARKMKDTDSEEEIREAFRVFDKDGNGYISAAELRHVMTNLGEKLTDEEVDEMIREA\n\
//! # DIDGDGQVNYEEFVQMMTAK*\n\
//! # >gi|5524211|gb|AAD44166.1| cytochrome b [Elephas maximus maximus]\n\
//! # LCLYTHIGRNIYYGSYLYSETWNTGIMLLLITMATAFMGYVLPWGQMSFWGATVITNLFSAIPYIGTNLV\n\
//! # EWIWGGFSVDKATLNRFFAFHFILPFTMVALAGVHLTFLHETGSNNPLGLTSDSDKIPFHPYYTIKDFLG\n\
//! # LLILILLLLLLALLSPDMLGDPDNHMPADPLNTPLHIKPEWYFLFAYAILRSVPNKLGGVLALFLSIVIL\n\
//! # GLMPFLHTSKHRSMMLRPLSQALFWTLTMDLLTLTWIGSQPVEYPYTIIGQMASILYFSIILAFLPIAGX\n\
//! # IENY\n\
//! # ";
//! assert!(file.p_arse(input).is_ok());
//! ```
//!
//! Example input:
//!
//! ```text
//! >MCHU - Calmodulin - Human, rabbit, bovine, rat, and chicken
//! MADQLTEEQIAEFKEAFSLFDKDGDGTITTKELGTVMRSLGQNPTEAELQDMINEVDADGNGTID
//! FPEFLTMMARKMKDTDSEEEIREAFRVFDKDGNGYISAAELRHVMTNLGEKLTDEEVDEMIREA
//! DIDGDGQVNYEEFVQMMTAK*
//! >gi|5524211|gb|AAD44166.1| cytochrome b [Elephas maximus maximus]
//! LCLYTHIGRNIYYGSYLYSETWNTGIMLLLITMATAFMGYVLPWGQMSFWGATVITNLFSAIPYIGTNLV
//! EWIWGGFSVDKATLNRFFAFHFILPFTMVALAGVHLTFLHETGSNNPLGLTSDSDKIPFHPYYTIKDFLG
//! LLILILLLLLLALLSPDMLGDPDNHMPADPLNTPLHIKPEWYFLFAYAILRSVPNKLGGVLALFLSIVIL
//! GLMPFLHTSKHRSMMLRPLSQALFWTLTMDLLTLTWIGSQPVEYPYTIIGQMASILYFSIILAFLPIAGX
//! IENY
//! ```

pub mod error;
pub mod function;
pub mod literal;
pub mod parser;
pub mod sequence;
pub mod utils;
pub mod wrapper;

pub use crate::{
    error::{Error, Result},
    function::{fun, rec, Fun},
    literal::CharExt,
    parser::Parser,
    sequence::TupleExt,
    utils::{any, eoi},
};
