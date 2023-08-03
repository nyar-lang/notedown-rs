// lints
#![warn(missing_docs)]
#![allow(clippy::needless_return)]
// documentation
#![doc = include_str!("../Readme.md")]

mod atomics;
pub mod text;
mod traits;

pub use crate::{
    atomics::{
        command::{CommandArguments, CommandBody, CommandNode},
        identifier::{AlignNode, IdentifierNode, LigatureNode, NumberLiteralNode, NumberValueNode},
        punctuation::{CommaNode, PeriodNode},
        whitespace::{HSpaceNode, IgnoreNode, NewlineNode, ParagraphSpaceNode, VSpaceNode, WhitespaceNode},
    },
    text::{ParagraphNode, ParagraphTerm, TextLiteralNode},
    traits::NotedownNode,
};
