use super::*;
use notedown_ast::traits::{TableOfContent, TocConfig, TocNode};
use notedown_error::NoteError;
use yggdrasil_shared::records::{
    lsp_types::{Diagnostic, DocumentSymbolResponse},
    TextIndex,
};

pub struct FileMeta {
    pub errors: Vec<NoteError>,
    pub toc: TocNode,
}

impl FileMeta {
    #[inline]
    pub fn clear(&mut self) {
        self.errors.clear();
        self.toc = TocNode::default()
    }
}

impl FileMeta {
    #[inline]
    pub fn push_lsp_diagnostics(&mut self, e: NoteError) {
        self.errors.push(e)
    }
    #[inline]
    pub fn as_lsp_diagnostics(&self, index: &TextIndex) -> Vec<Diagnostic> {
        self.errors.iter().map(|f| f.as_lsp_diagnostic(index)).collect()
    }
    #[inline]
    pub fn set_lsp_toc(&mut self, node: &ASTNode) {
        let cfg = TocConfig::default();
        self.toc = node.toc_configurable(&cfg);
    }
    #[inline]
    pub fn as_lsp_toc(&self, _: &TextIndex) -> DocumentSymbolResponse {
        todo!()
        // DocumentSymbolResponse::Nested(vec![self.toc.as_document_symbol(text)])
    }

    #[inline]
    pub fn can_gc(&self) -> bool {
        false
    }
}
