use super::*;

impl NativeChildren {
    pub fn new(parent: &Node<NativeSyntaxData>, reversed: bool) -> MaybeReversed<Self> {
        let current = if reversed { parent.last_child() } else { parent.first_child() };
        MaybeReversed { iterator: Self { parent: parent.clone(), current }, reversed }
    }
}

impl Iterator for NativeChildren {
    type Item = Node<NativeSyntaxData>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.as_ref() {
            Some(s) => {
                let ret = s.clone();
                self.current = s.next_sibling();
                Some(ret)
            }
            None => None,
        }
    }
}
impl DoubleEndedIterator for NativeChildren {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.current.as_ref() {
            Some(s) => {
                let ret = s.clone();
                self.current = s.previous_sibling();
                Some(ret)
            }
            None => None,
        }
    }
}
