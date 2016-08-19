use super::*;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum StyleKind {
    Plain = 0,

    Emphasis = 11,
    Strong = 13,
    ItalicBold = 14,

    Underline = 21,
    Undercover = 22,
    Marking = 23,
    Color(u8, u8, u8, u8) = 24,
    // HTMLColor(String) = 25,
    Delete = 31,
    Insert = 32,

    Subscript = 41,
    Superscript = 42,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct StyleNode {
    pub kind: StyleKind,
    pub children: ASTNodes,
}

impl From<&str> for StyleKind {
    fn from(style: &str) -> Self {
        match style {
            "*" | "i" | "italic" | "em" => Self::Emphasis,
            "**" | "b" | "bold" => Self::Strong,
            "***" => Self::ItalicBold,
            "~" | "u" | "underline" => Self::Underline,
            "~~" | "s" => Self::Delete,
            "~~~" => Self::Undercover,
            _ => Self::Plain,
        }
    }
}

impl StyleKind {
    pub fn surround_in(&self) -> &'static str {
        match self {
            Self::Plain => "",
            Self::Emphasis => "*",
            Self::Strong => "**",
            Self::ItalicBold => "***",
            Self::Underline => "~",
            Self::Delete => "~~",
            Self::Undercover => "~~~",
            Self::Marking => {
                unimplemented!()
            }
            Self::Insert => {
                unimplemented!()
            }
            Self::Color(_, _, _, _) => {
                unimplemented!()
            }
            Self::Subscript => "<sub>",
            Self::Superscript => "<sup>",
        }
    }
    pub fn surround_out(&self) -> &'static str {
        match self {
            Self::Plain => "",
            Self::Emphasis => "*",
            Self::Strong => "**",
            Self::ItalicBold => "***",
            Self::Underline => "~",
            Self::Delete => "~~",
            Self::Undercover => "~~~",
            Self::Marking => {
                unimplemented!()
            }
            Self::Insert => {
                unimplemented!()
            }
            Self::Color(_, _, _, _) => {
                unimplemented!()
            }
            Self::Subscript => "</sub>",
            Self::Superscript => "</sup>",
        }
    }
}

impl StyleNode {
    #[inline]
    pub fn into_node(self, range: Option<OffsetRange>) -> ASTNode {
        ASTNode { value: ASTKind::StyledSpan(box self), range }
    }
    #[inline]
    pub fn new(children: ASTNodes, style: &str) -> Self {
        Self { kind: StyleKind::from(style), children }
    }
}

macro_rules! styled_node {
    ($name:tt => $t:tt) => {
        impl StyleNode {
            #[inline]
            pub fn $name(children: ASTNodes) -> Self {
                Self { kind: StyleKind::$t, children }
            }
        }

        impl ASTKind {
            #[inline]
            pub fn $name(children: ASTNodes, range: Option<OffsetRange>) -> ASTNode {
                StyleNode::$name(children).into_node(range)
            }
        }
    };
    ($($name:tt => $t:tt),+ $(,)?) => (
        $(styled_node!($name=>$t);)+
    );
}

styled_node![
    bold        => Strong,
    strong      => Strong,
    italic      => Emphasis,
    emphasis    => Emphasis,
    italic_bold => ItalicBold,
    marking     => Marking,
    underline   => Underline,
    undercover  => Undercover,
    delete      => Delete,
    insert      => Insert,
    subscript   => Subscript,
    superscript => Superscript,
];
