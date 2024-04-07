use crate::{
    exports::notedown::core::{
        syntax_tree::NotedownRoot,
        types::{GuestUrl, NotedownError},
    },
    NotedownHost,
};
use std::str::FromStr;
use url::ParseError;

impl crate::exports::notedown::core::types::Guest for NotedownHost {
    type Url = UrlNative;
}

impl crate::exports::notedown::core::syntax_tree::Guest for NotedownHost {
    fn hack_unused() -> NotedownRoot {
        unreachable!()
    }
}

pub struct UrlNative {
    repr: url::Url,
}

impl GuestUrl for UrlNative {}

impl FromStr for UrlNative {
    type Err = NotedownError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { repr: url::Url::from_str(s)? })
    }
}

impl From<ParseError> for NotedownError {
    fn from(value: ParseError) -> Self {
        NotedownError::Unknown
    }
}
