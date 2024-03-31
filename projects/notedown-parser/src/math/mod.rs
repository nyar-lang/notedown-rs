use crate::{helpers::get_span, NoteParser};
use notedown_ast::ast::{InlineMathSpan, ParagraphSpan, ParagraphTerm};
use notedown_error::{ParseResult, ParseState};

impl NoteParser for InlineMathSpan {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("$")?;
        let (state, text) = ParagraphSpan::parse(state)?;
        let (state, _) = state.match_str("$")?;
        state.finish(Self { text, span: get_span(input, state) })
    }
}
