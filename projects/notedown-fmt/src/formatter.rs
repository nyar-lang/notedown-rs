use notedown_parser::{NoteDownParser, NoteDownRule as Rule};
use pangu::spacing;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use std::borrow::Cow;
use text_utils::{dedent_less_than, indent, indent_count};

macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub tab_size: usize,
    pub pangu_space: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { tab_size: 2, pangu_space: true }
    }
}

impl Settings {
    pub fn format(&self, text: &str) -> String {
        let input = text.replace("\t", &" ".repeat(self.tab_size)).replace("\n\r", "\n");
        self.format_program(&input)
    }
    pub fn format_program(&self, text: &str) -> String {
        let pairs = NoteDownParser::parse(Rule::program, text).unwrap_or_else(|e| panic!("{}", e));
        let mut codes = vec![];
        for pair in pairs {
            match pair.as_rule() {
                Rule::EOI => continue,
                Rule::NEWLINE => codes.push(String::from("\n")),
                Rule::SPACE_SEPARATOR => codes.push(String::from(" ")),

                Rule::Header => codes.push(self.format_header(pair)),
                Rule::TextBlock => codes.push(self.format_text(pair.as_str())),
                Rule::List => codes.push(self.format_list(pair)),
                Rule::CommandLine => codes.push(self.format_command_line(pair)),
                Rule::Code => codes.push(self.format_code(pair)),
                Rule::CommandBlock => codes.push(self.format_command_block(pair)),
                Rule::HorizontalRule => codes.push(pair.as_str().to_string()),
                _ => debug_cases!(pair),
            };
        }
        let mut out: String = codes.join("");
        if let Some(s) = out.chars().last() {
            if s != '\n' {
                out.push('\n');
            }
        }
        return out;
    }
    fn format_header(&self, pairs: Pair<Rule>) -> String {
        let mut level = 0;
        let mut text = String::new();
        for pair in pairs.into_inner() {
            let _code = match pair.as_rule() {
                Rule::SPACE_SEPARATOR => continue,
                Rule::Sharp => level += 1,
                Rule::RestOfLine => text = self.format_text(pair.as_str().trim()),
                _ => debug_cases!(pair),
            };
        }
        format!("{0} {1}", "#".repeat(level), text)
    }

    fn format_text(&self, input: &str) -> String {
        let text = if self.pangu_space { spacing(input) } else { Cow::from(input) };
        let spaces = indent_count(&text);
        let mut codes = vec![];
        for pair in parse_text(dedent_less_than(text.trim_end(), spaces).trim_end()) {
            match pair.as_rule() {
                Rule::EOI => continue,
                Rule::SPACE_SEPARATOR => codes.push(String::from(" ")),
                Rule::NEWLINE => codes.push(String::from("\n")),

                Rule::Raw => codes.push(pair.as_str().to_string()),
                Rule::URL => codes.push(pair.as_str().to_string()),
                Rule::Math => codes.push(self.format_math(pair)),
                Rule::Style => codes.push(self.format_style(pair)),

                Rule::TextRest => codes.push(pair.as_str().to_string()),
                Rule::StyleRest => codes.push(pair.as_str().to_string()),
                Rule::MathRest => codes.push(pair.as_str().to_string()),

                Rule::CommandBlock => codes.push(self.format_command_block(pair)),
                _ => debug_cases!(pair),
            };
        }
        if spaces == 0 { codes.join("") } else { format!("{}", indent(&codes.join(""), spaces)) }
    }
    fn format_style(&self, pairs: Pair<Rule>) -> String {
        let mut level = 0;
        let mut text = "";
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Asterisk => continue,
                Rule::StyleLevel => level = pair.as_str().len(),
                Rule::StyleText => text = pair.as_str().trim(),
                _ => debug_cases!(pair),
            };
        }
        format!("{0}{1}{0}", "*".repeat(level), text)
    }
    fn format_math(&self, pairs: Pair<Rule>) -> String {
        let mut level = 0;
        let mut text = "";
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Dollar => continue,
                Rule::MathLevel => level = pair.as_str().len(),
                Rule::MathText => text = pair.as_str().trim(),
                _ => debug_cases!(pair),
            };
        }
        format!("{0}{1}{0}", "$".repeat(level), text)
    }
    fn format_list(&self, input: Pair<Rule>) -> String {
        let text = input.as_str();
        let spaces = indent_count(text);
        let mut codes = vec![];

        for pair in input.into_inner() {
            match pair.as_rule() {
                Rule::SPACE_SEPARATOR => continue,
                Rule::ListMark => match pair.as_str() {
                    ">" => codes.push(self.format_quote(dedent_less_than(text, spaces).trim_end())),
                    _ => break,
                },
                _ => break,
            };
        }
        format!("{}", indent(&codes.join(""), spaces))
    }
    fn format_quote(&self, text: &str) -> String {
        let mut lines = vec![];
        for l in text.lines() {
            match l.chars().next().unwrap() {
                '>' => lines.push(&l[1..]),
                _ => lines.push(l),
            }
        }
        let f = self.format_text(&lines.join("\n"));
        let ls: Vec<String> = f.lines().map(|l| format!("> {}", l)).collect();
        return ls.join("\n");
        // let text = input.as_str();
        // let spaces = count_indent(text);
        // let mut codes = vec![];
        // for pair in parse_list(dedent_less_than(text, spaces).trim_end()) {
        // let code = match pair.as_rule() {
        // _ => debug_cases!(pair),
        // };
        // }
        // format!("{}", &indent(&codes.join(""), &" ".repeat(spaces)))
    }
    fn format_code(&self, input: Pair<Rule>) -> String {
        // let mut codes = vec![];
        let mut level = 0;
        let mut cmd = "";
        let mut txt = "";
        for pair in input.into_inner() {
            match pair.as_rule() {
                Rule::CodeMark => continue,
                Rule::CodeLevel => level = pair.as_str().len(),
                Rule::SYMBOL => cmd = pair.as_str(),
                Rule::CodeText => txt = pair.as_str().trim(),
                _ => debug_cases!(pair),
            };
        }
        format!("{0}{1}\n{2}\n{0}", "`".repeat(level), cmd, txt)
    }
    fn format_command_line(&self, input: Pair<Rule>) -> String {
        // let mut codes = vec![];
        let mut cmd = "";
        let mut rst = "";
        for pair in input.into_inner() {
            match pair.as_rule() {
                Rule::SPACE_SEPARATOR => continue,
                Rule::Colon => continue,
                Rule::command => cmd = pair.as_str(),
                Rule::RestOfLine => rst = pair.as_str().trim(),
                _ => unreachable!(),
            };
        }
        format!("{}: {}", cmd, rst)
    }
    fn format_command_block(&self, input: Pair<Rule>) -> String {
        // let mut codes = vec![];
        let mut cmd = "";
        let mut args = vec![];
        for pair in input.into_inner() {
            match pair.as_rule() {
                Rule::command => cmd = pair.as_str(),
                Rule::argument => args.push(pair.as_str()),
                _ => debug_cases!(pair),
            };
        }
        if args.len() != 0 { format!("{}{:?}", cmd, args) } else { String::from(cmd) }
    }
}

fn parse_text(text: &str) -> Pairs<Rule> {
    NoteDownParser::parse(Rule::TextMode, text).unwrap_or_else(|e| panic!("{}", e))
}

fn parse_table(text: &str) -> Pairs<Rule> {
    NoteDownParser::parse(Rule::TableMode, text).unwrap_or_else(|e| panic!("{}", e))
}

fn parse_list(text: &str) -> Pairs<Rule> {
    let p = NoteDownParser::parse(Rule::ListMode, text).unwrap_or_else(|e| panic!("{}", e));
    p.into_iter().next().unwrap().into_inner()
}
// #[derive(Debug)]
// enum List {
// Quote,
// Ordered,
// Orderless,
// }
//
// impl List {}