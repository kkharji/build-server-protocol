use super::BuildTargetIdentifier;
use lsp_types::TextDocumentIdentifier;
use serde::{Deserialize, Serialize};

/// The build target dependency sources request is sent from the client to the server to query for
/// the sources of build target dependencies that are external to the workspace. The dependency
/// sources response must not include source files that belong to a build target within the
/// workspace, see buildTarget/sources.

/// The server communicates during the initialize handshake whether this method is supported or
/// not. This method can for example be used by a language server on textDocument/definition to "Go
/// to definition" from project sources to dependency sources.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BuildTargetDependencySources {
    text_document: TextDocumentIdentifier,
}

impl BuildTargetDependencySources {
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
pub struct BuildTargetDependencySourcesResult {
    items: Vec<DependencySourcesItem>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DependencySourcesItem {
    target: BuildTargetIdentifier,
    /// List of resources containing source files of the
    ///    * target's dependencies.
    ///    * Can be source files, jar files, zip files, or directories.
    sources: Vec<String>,
}

impl DependencySourcesItem {
    /// Get a reference to the dependency sources item's sources.
    ///
    /// List of resources containing source files of the
    ///    * target's dependencies.
    ///    * Can be source files, jar files, zip files, or directories.
    pub fn sources(&self) -> &[String] {
        self.sources.as_ref()
    }

    /// Get a reference to the dependency sources item's target.
    pub fn target(&self) -> &BuildTargetIdentifier {
        &self.target
    }

    /// Set the dependency sources item's target.
    pub fn set_target(&mut self, target: BuildTargetIdentifier) {
        self.target = target;
    }

    /// Set the dependency sources item's sources.
    pub fn set_sources(&mut self, sources: Vec<String>) {
        self.sources = sources;
    }
}
