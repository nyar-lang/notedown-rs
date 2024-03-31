use crate::exports::peg::core::cst::SnytaxFlags;
use rctree::Node;
use std::{fmt::Debug, ops::Range, rc::Rc};
use yggdrasil_rt::{TokenPair, YggdrasilRule};

#[derive(Debug, Clone)]
pub struct NativeLanguage {
    pub name: &'static str,
    pub glob: &'static [&'static str],
}

#[derive(Debug, Clone)]
pub struct NativeSyntaxRule {
    pub language: NativeLanguage,
    pub flags: SnytaxFlags,
    pub name: &'static str,
    pub styles: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct NativeSyntaxData {
    pub rule: NativeSyntaxRule,
    pub tag: String,
    pub raw: Rc<str>,
    pub span: Range<usize>,
}

impl NativeSyntaxData {
    pub fn new<R: YggdrasilRule + 'static>(
        input: Rc<str>,
        pair: TokenPair<R>,
        language: &NativeLanguage,
    ) -> Node<NativeSyntaxData> {
        let rule = pair.get_rule();
        let parent = Node::new(NativeSyntaxData {
            rule: NativeSyntaxRule {
                language: language.clone(),
                flags: SnytaxFlags::empty(),
                name: rule.get_name(),
                styles: vec![rule.get_style().to_string()],
            },
            tag: pair.get_tag().to_string(),
            raw: input.clone(),
            span: pair.get_span().get_range(),
        });
        for child in pair.into_inner() {
            parent.append(NativeSyntaxData::new(input.clone(), child, language))
        }
        parent
    }
}
