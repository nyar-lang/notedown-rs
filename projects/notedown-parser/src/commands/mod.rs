use crate::{
    helpers::{get_span, rest_of_line},
    NoteParser,
};
use notedown_ast::{
    ast::{CodeInlineSpan, CommandLineSpan, NewlineSpan, ParagraphTerm, TextSpaceNode},
    hir::{EscapeNode, IdentifierNode, TextPlainNode},
};
use notedown_error::{ParseResult, ParseState};

pub fn parse_command(input: ParseState) -> ParseResult<ParagraphTerm> {
    let (state, _) = input.match_char('\\')?;
    let (state, id) = IdentifierNode::parse(state)?;
    state
        .begin_choice()
        .choose(|s| {
            let (finally, rest) = parse_cmd_rol(s)?;
            finally.finish(CommandLineSpan { command: id.clone(), rest, span: get_span(input, finally) }.into())
        })
        .choose(|s| {
            let (finally, rest) = parse_cmd_row(s)?;
            finally.finish(CommandLineSpan { command: id.clone(), rest, span: get_span(input, finally) }.into())
        })
        .end_choice()
}

/// `\command: rest of line`
fn parse_cmd_rol(input: ParseState) -> ParseResult<TextPlainNode> {
    let (state, _) = input.skip(TextSpaceNode::parse).match_char(':')?;
    let (state, text) = rest_of_line(state)?;
    state.finish(TextPlainNode { text: text.to_string(), span: get_span(input, state) })
}

/// ``\command`code``
fn parse_cmd_row(input: ParseState) -> ParseResult<TextPlainNode> {
    let state = input.skip(NewlineSpan::parse);
    let (state, text) = CodeInlineSpan::parse(state)?;
    state.finish(TextPlainNode { text: text.code, span: get_span(input, state) })
}
