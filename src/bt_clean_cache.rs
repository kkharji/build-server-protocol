use serde::{Deserialize, Serialize};

use crate::BuildTargetIdentifier;

/// The clean cache request is sent from the client to the server to reset any state associated with
/// a given build target. The state can live either in the build tool or in the file system.
///
/// The build tool defines the exact semantics of the clean cache request:
///
/// Stateless build tools are free to ignore the request and respond with a successful response.
/// Stateful build tools must ensure that invoking compilation on a target that has been cleaned
/// results in a full compilation.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BTCleanCacheParams {
    /// The build targets to clean.
    targets: Vec<BuildTargetIdentifier>,
}

impl BTCleanCacheParams {
    /// Get a reference to the bsp btclean cache params's targets.
    pub fn targets(&self) -> &[BuildTargetIdentifier] {
        self.targets.as_ref()
    }

    /// Get a mutable reference to the bsp btclean cache params's targets.
    pub fn targets_mut(&mut self) -> &mut Vec<BuildTargetIdentifier> {
        &mut self.targets
    }

    pub fn is_empty(&self) -> bool {
        self.targets.is_empty()
    }

    /// Set the bsp btclean cache params's targets.
    pub fn set_targets(&mut self, targets: Vec<BuildTargetIdentifier>) {
        self.targets = targets;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BTCleanCacheResult {
    /// Optional message to display to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    /// Indicates whether the clean cache request was performed or not.
    cleaned: bool,
}

impl BTCleanCacheResult {
    pub fn new(message: Option<String>, cleaned: bool) -> Self {
        Self { message, cleaned }
    }
    pub fn new_simple(cleaned: bool) -> Self {
        Self {
            message: None,
            cleaned,
        }
    }

    /// Get a reference to the clean cache result's message.
    pub fn message(&self) -> Option<&String> {
        self.message.as_ref()
    }

    /// Get the clean cache result's cleaned.
    pub fn cleaned(&self) -> bool {
        self.cleaned
    }

    /// Set the clean cache result's cleaned.
    pub fn set_cleaned(&mut self, cleaned: bool) {
        self.cleaned = cleaned;
    }

    /// Set the clean cache result's message.
    pub fn set_message(&mut self, message: Option<String>) {
        self.message = message;
    }
}
