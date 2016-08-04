mod elements;
mod literal;
mod value;

pub use self::{
    elements::*,
    literal::Literal,
    value::{Array, Object, Set, Value, ValueType},
};
pub use indexmap::map::{Keys, Values};
use indexmap::{map::IndexMap, set::IndexSet};
use num::{BigInt, BigUint};
use std::{
    collections::BTreeMap,
    fmt::{self, Debug, Display, Formatter},
    hash::{Hash, Hasher},
    mem::transmute,
};

pub type ASTNode = Literal<ASTKind>;
pub type ASTNodes = Vec<Literal<ASTKind>>;

/// Block,
/// - Block:
/// - Span: Text, Styled
/// - Node: Code, Math, Link, Command
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ASTKind {
    /// Top Scope
    Statements(ASTNodes),
    ///  - `Paragraph`:
    Paragraph(ASTNodes),
    /// block
    Delimiter(Box<Delimiter>),
    // Blocks
    /// - `Header`: TEXT, level
    Header(Box<Header>),
    ///
    TableView(Box<TableView>),
    ///
    ListView(Box<ListView>),
    /// block: ``` a ```
    /// span: `` `code`  ``
    CodeNode(Box<CodeNode>),
    /// block: ``` a ```
    /// span: `` `code`  ``
    MathNode(Box<MathNode>),
    /// block: ``` a ```
    /// span: `` `code`  ``
    LinkNode(Box<SmartLink>),
    /// span
    TextSpan(Box<TextNode>),
    /// span
    StyledSpan(Box<StyleNode>),
    /// in
    Command(Box<Command>),
    Value(Box<Value>),
}

impl Default for ASTKind {
    fn default() -> Self {
        Self::Value(Box::new(Value::Null))
    }
}

impl ASTKind {
    #[inline]
    pub fn into_node(self, range: Option<(u32, u32)>) -> ASTNode {
        ASTNode { value: self, range }
    }
    #[inline]
    pub fn statements(children: ASTNodes, range: Option<(u32, u32)>) -> ASTNode {
        ASTNode { value: Self::Statements(children), range }
    }
    #[inline]
    pub fn paragraph(children: ASTNodes, range: Option<(u32, u32)>) -> ASTNode {
        ASTNode { value: Self::Paragraph(children), range }
    }
    #[inline]
    pub fn header(children: ASTNodes, level: u8, range: Option<(u32, u32)>) -> ASTNode {
        Header { level, children }.into_node(range)
    }
    #[inline]
    pub fn text(s: impl Into<String>, range: Option<(u32, u32)>) -> ASTNode {
        TextNode::Normal(s.into()).into_node(range)
    }
    #[inline]
    pub fn bold(children: ASTNodes, range: Option<(u32, u32)>) -> ASTNode {
        StyleNode { kind: StyleKind::Bold, children }.into_node(range)
    }
    #[inline]
    pub fn italic(children: ASTNodes, range: Option<(u32, u32)>) -> ASTNode {
        StyleNode { kind: StyleKind::Italic, children }.into_node(range)
    }
    #[inline]
    pub fn emphasis(children: ASTNodes, range: Option<(u32, u32)>) -> ASTNode {
        StyleNode { kind: StyleKind::Emphasis, children }.into_node(range)
    }
    #[inline]
    pub fn underline(children: ASTNodes, range: Option<(u32, u32)>) -> ASTNode {
        StyleNode { kind: StyleKind::Underline, children }.into_node(range)
    }
    #[inline]
    pub fn strikethrough(children: ASTNodes, range: Option<(u32, u32)>) -> ASTNode {
        StyleNode { kind: StyleKind::Strikethrough, children }.into_node(range)
    }
    #[inline]
    pub fn undercover(children: ASTNodes, range: Option<(u32, u32)>) -> ASTNode {
        StyleNode { kind: StyleKind::Undercover, children }.into_node(range)
    }
    #[inline]
    pub fn hr(range: Option<(u32, u32)>) -> ASTNode {
        ASTNode { value: Self::Delimiter(Box::new(Delimiter::HorizontalRule)), range }
    }
}
