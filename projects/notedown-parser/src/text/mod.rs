mod link;
mod style;

use crate::{commands::parse_command, helpers::get_span, traits::NoteParser};
use notedown_ast::{
    ast::{
        CodeInlineSpan, CommaNode, FontBoldItalicSpan, FontBoldSpan, FontDeleteSpan, FontItalicSpan, FontUnderlineSpan, HeadingSpan,
        NewlineSpan, NotedownTerm, ParagraphBreakSpan, ParagraphSpan, ParagraphTerm, PeriodNode, TextSpaceNode,
    },
    hir::{EscapeNode, IdentifierNode, TextPlainNode, UriNode},
};
use notedown_error::{helpers::paragraph_break, ParseResult, ParseState, StopBecause};

impl NoteParser for NotedownTerm {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .choose_from(ParagraphBreakSpan::parse)
            .choose_from(HeadingSpan::parse)
            .choose_from(ParagraphSpan::parse)
            .end_choice()
    }
}

impl NoteParser for EscapeNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_char('\\')?;
        let (state, any) = state.match_char_any()?;
        state.finish(Self { escape: any, span: get_span(input, state) })
    }
}

impl NoteParser for ParagraphSpan {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, terms) = input.match_repeats(ParagraphTerm::parse)?;
        if terms.is_empty() {
            StopBecause::missing_string("TEXT_MODE_TERM", input.start_offset)?
        }
        state.finish(Self { terms, span: get_span(input, state) })
    }
}

impl NoteParser for ParagraphBreakSpan {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = paragraph_break(input)?;
        state.finish(Self { span: get_span(state, state) })
    }
}

impl NoteParser for ParagraphTerm {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_negative(ParagraphBreakSpan::parse, "PARAGRAPH_BREAK")?;
        state
            .begin_choice()
            .choose_from(CodeInlineSpan::parse)
            .choose_from(FontBoldItalicSpan::parse)
            .choose_from(FontBoldSpan::parse)
            .choose_from(FontItalicSpan::parse)
            .choose_from(FontDeleteSpan::parse)
            .choose_from(FontUnderlineSpan::parse)
            .choose_from(UriNode::parse)
            .choose(parse_command)
            .choose_from(EscapeNode::parse)
            .choose_from(TextPlainNode::parse)
            .choose_from(CommaNode::parse)
            .choose_from(PeriodNode::parse)
            .choose_from(TextSpaceNode::parse)
            .choose_from(NewlineSpan::parse)
            .end_choice()
    }
}

impl NoteParser for CommaNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_char(',')?;
        state.finish(Self { span: get_span(input, state) })
    }
}

impl NoteParser for PeriodNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_char('.')?;
        state.finish(Self { span: get_span(input, state) })
    }
}

impl NoteParser for HeadingSpan {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, mark) = input.match_str_if(|c| c == '=', "TITLE_MARK")?;
        let (state, _) = TextSpaceNode::parse(state)?;
        let (state, text) = ParagraphSpan::parse(state)?;
        state.finish(Self { level: mark.len(), text, span: get_span(input, state) })
    }
}
