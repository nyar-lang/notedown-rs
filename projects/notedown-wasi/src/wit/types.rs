use super::*;
use yggdrasil_rt::{errors::YggdrasilErrorKind, YggdrasilError, YggdrasilRule};

use crate::exports::peg::core::types::*;

impl Guest for YggdrasilHost {
    type Language = NativeLanguage;
}

impl GuestLanguage for NativeLanguage {
    fn get_language_name(&self) -> String {
        self.name.to_string()
    }

    fn get_glob_pattern(&self) -> Vec<String> {
        self.glob.iter().map(|s| s.to_string()).collect()
    }
}

impl<R: YggdrasilRule> From<YggdrasilError<R>> for ParseError {
    fn from(value: YggdrasilError<R>) -> Self {
        let range = TextRange { head_offset: value.location.start as u32, tail_offset: value.location.end as u32 };
        match value.variant {
            YggdrasilErrorKind::InvalidRule { .. } => ParseError::Custom(CustomError { message: String::new(), range }),
            YggdrasilErrorKind::InvalidNode { .. } => ParseError::Custom(CustomError { message: String::new(), range }),
            YggdrasilErrorKind::InvalidTag { .. } => ParseError::Custom(CustomError { message: String::new(), range }),
            YggdrasilErrorKind::CustomError { message } => ParseError::Custom(CustomError { message, range }),
        }
    }
}
