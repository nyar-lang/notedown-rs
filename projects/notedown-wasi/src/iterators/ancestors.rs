use super::*;

impl NativeAncestors {
    pub fn new(head: &Node<NativeSyntaxData>, include_self: bool) -> MaybeReversed<Self> {
        MaybeReversed {
            iterator: Self { remember: vec![head.clone()], current: if include_self { 0 } else { 1 } },
            reversed: false,
        }
    }
}

impl Iterator for NativeAncestors {
    type Item = Node<NativeSyntaxData>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.remember.len() {
            let last = unsafe { self.remember.get_unchecked(self.current - 1) };
            match last.parent() {
                Some(s) => {
                    self.remember.push(s.clone());
                }
                None => {
                    return None;
                }
            }
        }
        let result = unsafe { self.remember.get_unchecked(self.current).clone() };
        self.current += 1;
        Some(result)
    }
}

impl DoubleEndedIterator for NativeAncestors {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current == 0 {
            return None;
        }
        self.current -= 1;
        unsafe { Some(self.remember.get_unchecked(self.current).clone()) }
    }
}
