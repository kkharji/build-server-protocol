use lsp_types::TextDocumentIdentifier;
use serde::{Deserialize, Serialize};

use super::BuildTargetIdentifier;

/// The inverse sources request is sent from the client to the server to query for the list of
/// build targets containing a text document. The server communicates during the initialize
/// handshake whether this method is supported or not. This request can be viewed as the inverse of
/// buildTarget/sources, except it only works for text documents and not directories.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentInverseSources {
    text_document: TextDocumentIdentifier,
}

impl TextDocumentInverseSources {
    pub fn new(text_document: TextDocumentIdentifier) -> Self {
        Self { text_document }
    }

    /// Get a reference to the bsp inverse sources params's text document.
    pub fn text_document(&self) -> &TextDocumentIdentifier {
        &self.text_document
    }

    /// Set the bsp inverse sources params's text document.
    pub fn set_text_document(&mut self, text_document: TextDocumentIdentifier) {
        self.text_document = text_document;
    }
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuildTargetInverseSourcesResult {
    targets: Vec<BuildTargetIdentifier>,
}
