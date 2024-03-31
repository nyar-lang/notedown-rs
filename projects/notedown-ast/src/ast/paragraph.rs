use super::*;
use crate::hir::EscapeNode;

/// Sequence of paragraph ast
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParagraphSpan {
    pub terms: Vec<ParagraphTerm>,
    pub span: Range<u32>,
}

/// Item of paragraph ast
#[derive(Clone, Eq, PartialEq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ParagraphTerm {
    /// Normal ast with white space
    Text(Box<TextPlainNode>),
    /// `*italic*`
    Italic(Box<FontItalicSpan>),
    /// `**bold**`
    Bold(Box<FontBoldSpan>),
    /// `**bold italic**`
    BoldItalic(Box<FontBoldItalicSpan>),
    /// `~underline~`
    Underline(Box<FontUnderlineSpan>),
    /// `~~delete~~`
    Delete(Box<FontDeleteSpan>),
    /// `` `code` ``
    Code(Box<CodeInlineSpan>),
    /// `$$display math$$`
    DisplayMath(Box<DisplayMathSpan>),
    /// `$inline math$`
    InlineMath(Box<InlineMathSpan>),
    /// `https://example.com`
    Uri(Box<UriNode>),
    /// `\cmd: rest of the line`
    CommandLine(Box<CommandLineSpan>),
    /// ` `
    WhiteSpace(Box<TextSpaceNode>),
    /// `\n`
    NewLine(Box<NewlineSpan>),
    /// `,`
    Comma(Box<CommaNode>),
    /// `.`
    Period(Box<PeriodNode>),
    /// `\\`
    Escape(Box<EscapeNode>),
}

impl ParagraphSpan {
    pub fn as_hir(&self) -> ParagraphNode {
        let mut terms: Vec<ParagraphKind> = Vec::with_capacity(self.terms.len());
        for term in &self.terms {
            match term {
                ParagraphTerm::Text(v) => terms.push(v.as_ref().clone().into()),
                ParagraphTerm::WhiteSpace(v) => {
                    terms.push(ParagraphKind::Space(v.clone()));
                }
                ParagraphTerm::Italic(v) => {
                    terms.push(v.as_hir().into());
                }
                ParagraphTerm::Bold(v) => {
                    terms.push(v.as_hir().into());
                }
                ParagraphTerm::BoldItalic(v) => terms.push(v.as_hir().into()),
                ParagraphTerm::NewLine(v) => {
                    terms.push(v.as_hir().into());
                }
                ParagraphTerm::Comma(v) => {
                    terms.push(v.as_hir().into());
                }
                ParagraphTerm::Period(v) => {
                    terms.push(v.as_hir().into());
                }
                ParagraphTerm::Escape(v) => {
                    match v.escape {
                        // skip \s, \r, \n
                        c if c.is_ascii_whitespace() => continue,
                        'n' => terms.push(ParagraphKind::text('\n', v.span.clone())),
                        't' => terms.push(ParagraphKind::text('\t', v.span.clone())),
                        _ => terms.push(ParagraphKind::text(v.escape, v.span.clone())),
                    }
                    // terms.push(v.as_hir().into());
                }
                ParagraphTerm::Underline(v) => terms.push(v.as_hir().into()),
                ParagraphTerm::Delete(v) => terms.push(v.as_hir().into()),
                ParagraphTerm::Code(v) => terms.push(v.as_hir().into()),
                ParagraphTerm::CommandLine(v) => terms.push(v.as_hir().into()),
                ParagraphTerm::Uri(v) => terms.push(ParagraphKind::Uri(v.clone())),
                ParagraphTerm::DisplayMath(v) => terms.push(v.as_hir().into()),
                ParagraphTerm::InlineMath(v) => terms.push(v.as_hir().into()),
            }
        }
        ParagraphNode { terms, span: self.span.clone() }
    }
}
