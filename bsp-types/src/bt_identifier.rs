use lsp_types::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct BuildTargetIdentifier {
    uri: Url,
}

impl BuildTargetIdentifier {
    pub fn new(uri: Url) -> Self {
        Self { uri }
    }

    /// Get a reference to the bsp build target identifier's uri.
    pub fn uri(&self) -> &str {
        self.uri.as_ref()
    }

    /// Set the bsp build target identifier's uri.
    pub fn set_uri(&mut self, uri: Url) {
        self.uri = uri;
    }
}
