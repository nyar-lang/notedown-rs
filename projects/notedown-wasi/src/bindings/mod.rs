use crate::{
    exports::notedown::core::{syntax_tree::NotedownRoot, types::GuestUrl},
    NotedownHost,
};

impl crate::exports::notedown::core::types::Guest for NotedownHost {
    type Url = UrlNative;
}

impl crate::exports::notedown::core::syntax_tree::Guest for NotedownHost {
    fn hack_unused() -> NotedownRoot {
        todo!()
    }
}

pub struct UrlNative {}

impl GuestUrl for UrlNative {}
