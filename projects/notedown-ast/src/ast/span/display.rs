use crate::Span;
use std::fmt::{Display, Formatter};
use std::fmt;

impl<'a> Display for Span<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Span::String(s) => write!(f, "{}", s),
            Span::Bold(s) => {unimplemented!()}
            Span::Italic(s) => write!(f, "*{}*", s),
            Span::Underline(s) => {unimplemented!()}
            Span::Strikethrough(s) => {unimplemented!()}
            Span::Undercover(s) => {unimplemented!()}
            Span::MathInline(_) => {unimplemented!()}
            Span::MathDisplay(_) => {unimplemented!()}
        }
    }
}