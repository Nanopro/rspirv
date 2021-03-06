//! Module for SPIR-V binary processing.
//!
//! This module provides a [`Decoder`](struct.Decoder.html) and a
//! [`Parser`](struct.Parser.html):
//!
//! * The decoder is a low-level binary processing tool; it has no knowlege
//!   of the SPIR-V grammar. It only serves SPIR-V word requests.
//! * The parser is a high-level binary processing tool; it has knowledge
//!   of the SPIR-V grammar. It works with the
//!   [`Consumer`](trait.Consumer.html) to process a SPIR-V binary on the
//!   instruction level.

pub use self::decoder::Decoder;
pub use self::autogen_error::Error as DecodeError;
pub use self::parser::{Consumer, parse_bytes, parse_words, Parser};
pub use self::parser::Action as ParseAction;
pub use self::parser::Result as ParseResult;
pub use self::parser::State as ParseState;

pub use self::disassemble::Disassemble;
pub use self::assemble::Assemble;

mod assemble;
mod decoder;
mod disassemble;
mod autogen_error;
mod parser;
mod tracker;
