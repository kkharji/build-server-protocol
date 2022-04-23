use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct BuildTargetIdentifier {
    uri: String,
}

impl BuildTargetIdentifier {
    pub fn new(uri: String) -> Self {
        Self { uri }
    }

    /// Get a reference to the bsp build target identifier's uri.
    pub fn uri(&self) -> &str {
        self.uri.as_ref()
    }

    /// Set the bsp build target identifier's uri.
    pub fn set_uri(&mut self, uri: String) {
        self.uri = uri;
    }

    pub fn is_empty(&self) -> bool {
        self.uri.is_empty()
    }
}
