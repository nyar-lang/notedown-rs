// features
#![feature(box_syntax)]
#![feature(map_first_last)]
#![feature(arbitrary_enum_discriminant)]
// lints
#![deny(missing_docs)]
#![allow(clippy::needless_return)]
// documentation
#![doc = include_str!("../Readme.md")]
pub mod command;
mod errors;
#[cfg(feature = "lsp")]
mod language_server;
pub mod nodes;
pub mod traits;
pub mod value;

pub use self::{
    command::Command,
    errors::{DiagnosticLevel, NoteError, NoteErrorKind, Result},
    nodes::{ASTKind, ASTNode, ASTNodes},
    value::Value,
};

pub mod utils {
    //! Auxiliary tools and related libraries
    pub use indexmap;
    pub use itertools;
    pub use text_utils;
    #[cfg(feature = "lsp")]
    mod lsp_wrap {
        pub use yggdrasil_shared::records::{lsp_types, DashMap, DashSet, LSPPosition, LSPRange, Rope, TextIndex, Url};
    }
    #[cfg(feature = "lsp")]
    pub use lsp_wrap::*;
}
