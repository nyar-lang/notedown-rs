use super::*;

use crate::exports::peg::core::cst::*;

impl Guest for YggdrasilHost {
    type SyntaxRule = NativeSyntaxRule;
    type SyntaxNode = Node<NativeSyntaxData>;
    type SyntaxIterator = NativeSyntaxIterator;
}

impl GuestSyntaxRule for NativeSyntaxRule {
    fn get_flags(&self) -> SnytaxFlags {
        todo!()
    }

    fn get_language(&self) -> Language {
        todo!()
    }

    fn get_rule_name(&self) -> String {
        self.name.to_string()
    }

    fn get_styles(&self) -> Vec<String> {
        self.styles.iter().map(|s| s.to_string()).collect()
    }
}

impl GuestSyntaxNode for Node<NativeSyntaxData> {
    fn get_range(&self) -> TextRange {
        let data = self.borrow();
        let range = data.span.clone();
        TextRange { head_offset: range.start as u32, tail_offset: range.end as u32 }
    }
    fn get_tag(&self) -> String {
        self.borrow().tag.clone()
    }
    fn has_tag(&self, tag: String) -> bool {
        self.borrow().tag.eq(&tag)
    }
    fn get_rule(&self) -> SyntaxRule {
        SyntaxRule::new(self.borrow().rule.clone())
    }

    fn get_text(&self) -> String {
        let data = self.borrow();
        let range = data.span.clone();
        match data.raw.get(range) {
            Some(s) => s.to_string(),
            None => {
                panic!("out of range")
            }
        }
    }

    fn has_parent(&self) -> bool {
        self.parent().is_some()
    }

    fn get_parent(&self) -> Option<SyntaxNode> {
        Some(SyntaxNode::new(self.parent()?))
    }

    fn get_ancestors(&self, include_self: bool) -> SyntaxIterator {
        SyntaxIterator::new(NativeSyntaxIterator::Ancestors(RefCell::new(NativeAncestors::new(self, include_self))))
    }

    fn get_last(&self) -> Option<SyntaxNode> {
        Some(SyntaxNode::new(self.previous_sibling()?))
    }

    fn get_last_iterator(&self, include_self: bool) -> SyntaxIterator {
        todo!()
    }

    fn get_next(&self) -> Option<SyntaxNode> {
        Some(SyntaxNode::new(self.next_sibling()?))
    }

    fn get_next_iterator(&self, include_self: bool) -> SyntaxIterator {
        todo!()
    }

    fn get_sibling_head(&self) -> Option<SyntaxNode> {
        Some(SyntaxNode::new(self.parent()?.first_child()?))
    }

    fn get_sibling_tail(&self) -> Option<SyntaxNode> {
        Some(SyntaxNode::new(self.parent()?.last_child()?))
    }

    fn get_siblings(&self, reversed: bool) -> SyntaxIterator {
        todo!()
    }

    fn has_child(&self) -> bool {
        self.has_children()
    }

    fn count_children(&self) -> u32 {
        todo!()
    }

    fn get_child_head(&self) -> Option<SyntaxNode> {
        Some(SyntaxNode::new(self.first_child()?))
    }

    fn get_child_tail(&self) -> Option<SyntaxNode> {
        Some(SyntaxNode::new(self.last_child()?))
    }

    fn get_children(&self, reversed: bool) -> SyntaxIterator {
        SyntaxIterator::new(NativeSyntaxIterator::Children(RefCell::new(NativeChildren::new(self, reversed))))
    }

    fn get_descendants(&self, depth_first: bool, reversed: bool) -> SyntaxIterator {
        todo!()
    }
}

impl GuestSyntaxIterator for NativeSyntaxIterator {
    fn last(&self) -> Option<SyntaxNode> {
        match self {
            Self::Ancestors(i) => Some(SyntaxNode::new(i.borrow_mut().backward()?)),
            Self::Siblings() => {
                todo!()
            }
            Self::Previous() => {
                todo!()
            }
            Self::Following() => {
                todo!()
            }
            Self::Children(i) => Some(SyntaxNode::new(i.borrow_mut().backward()?)),
            Self::Descendants() => {
                todo!()
            }
        }
    }

    fn next(&self) -> Option<SyntaxNode> {
        match self {
            Self::Ancestors(i) => Some(SyntaxNode::new(i.borrow_mut().forward()?)),
            Self::Siblings() => {
                todo!()
            }
            Self::Previous() => {
                todo!()
            }
            Self::Following() => {
                todo!()
            }
            Self::Children(i) => Some(SyntaxNode::new(i.borrow_mut().forward()?)),
            Self::Descendants() => {
                todo!()
            }
        }
    }

    fn move_head(&self) {
        todo!()
    }

    fn move_tail(&self) {
        todo!()
    }

    fn skip(&self, count: u32) {
        todo!()
    }

    fn reverse(&self) {
        match self {
            Self::Ancestors(v) => v.borrow_mut().reverse(),
            Self::Siblings() => {}
            Self::Previous() => {}
            Self::Following() => {}
            Self::Children(v) => v.borrow_mut().reverse(),
            Self::Descendants() => {}
        }
    }
}
