use super::*;
use crate::atomics::whitespace::WhitespaceSpan;
use std::fmt::{Debug, Formatter};

/// CommandNode
///
/// ```note
/// \cmd () { }
/// ```
#[derive(Debug)]
pub struct CommandNode {
    name: String,
    span: Range<u32>,
}

/// CommandNode
///
/// ```note
/// ()
/// ```
#[derive(Debug)]
pub struct CommandArguments {
    prefill: Option<WhitespaceSpan>,
    span: Range<u32>,
}

/// CommandNode
///
/// ```note
/// { }
/// ```
#[derive(Debug)]
pub struct CommandBody {}

impl Display for CommandArguments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for CommandNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\\{}", self.name)
    }
}

impl CommandNode {
    pub fn new<S: ToString>(body: S, span: Range<u32>) -> Self {
        Self { name: body.to_string(), span }
    }
}

impl CommandArguments {
    pub fn with_prefill(self, space: Option<WhitespaceSpan>) -> Self {
        Self { prefill: space, ..self }
    }
}
