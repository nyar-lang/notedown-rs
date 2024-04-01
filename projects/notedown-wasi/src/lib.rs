#![allow(unused_braces)]

pub struct NotedownHost {}

pub use crate::bindings::UrlNative;

include!("exports/host.rs");
export!(NotedownHost);

mod bindings;
mod traits;
