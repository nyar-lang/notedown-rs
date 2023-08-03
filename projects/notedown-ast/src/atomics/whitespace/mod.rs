pub use super::*;
mod display;

#[derive(Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IgnoreNode {
    /// Whitespace
    WS(WhitespaceNode),
    /// Newline
    NL(NewlineNode),
}

/// A period of whitespace longer than two newlines, terminated by a newline
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParagraphSpaceNode {
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhitespaceNode {
    pub width: usize,
    pub span: Range<u32>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HSpaceNode {
    pub width: f32,
    pub span: Range<u32>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VSpaceNode {
    pub height: f32,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NewlineNode {
    pub count: usize,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AlignNode {
    pub span: Range<u32>,
}

impl WhitespaceNode {
    /// Create a new whitespace node with the given width and span.
    pub fn new(width: usize, span: Range<u32>) -> Self {
        Self { width, span }
    }
}

impl NewlineNode {
    /// Create a new newline node with the given line count and span.
    pub fn new(lines: usize, span: Range<u32>) -> Self {
        Self { count: lines, span }
    }
}
