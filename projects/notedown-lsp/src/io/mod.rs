use lspower::lsp_types::Url;
use std::fs;

mod global;
pub use global::{initialize_global_storages, FileStateUpdate, FILE_STORAGE};

pub fn read_url(url: &Url) -> String {
    url.to_file_path().ok().and_then(|e| fs::read_to_string(e).ok()).unwrap_or_default()
}
