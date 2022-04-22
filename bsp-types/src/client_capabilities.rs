use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClientCapabilities {
    /// The languages that this client supports.
    /// The ID strings for each language is defined in the LSP.
    /// The server must never respond with build targets for other
    /// languages than those that appear in this list.
    language_ids: Vec<String>,
}

impl ClientCapabilities {
    pub fn new(language_ids: Vec<String>) -> Self {
        Self { language_ids }
    }

    /// Set the bsp client capabilities's language ids.
    pub fn set_language_ids(&mut self, language_ids: Vec<String>) {
        self.language_ids = language_ids;
    }

    /// Get a reference to the bsp client capabilities's language ids.
    pub fn language_ids(&self) -> &[String] {
        self.language_ids.as_ref()
    }

    /// Get a mutable reference to the bsp client capabilities's language ids.
    pub fn language_ids_mut(&mut self) -> &mut Vec<String> {
        &mut self.language_ids
    }
}
