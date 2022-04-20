use super::BspBuildTargetIdentifier;
use serde::{Deserialize, Serialize};

/// The build target changed notification is sent from the server to the client
/// to signal a change in a build target. The server communicates during the
/// initialize handshake whether this method is supported or not.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct BuildTargetsChangedNotification {
    pub changes: Vec<BuildTargetEvent>,
}

impl BuildTargetsChangedNotification {
    pub const METHOD: &'static str = "buildTarget/didChange";
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct BuildTargetEvent {
    /// The identifier for the changed build target.
    pub target: BspBuildTargetIdentifier,
    /// The kind of change for this build target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<BuildTargetEventKind>,
    /// Any additional metadata about what information changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl BuildTargetEvent {
    pub fn new(
        target: BspBuildTargetIdentifier,
        kind: Option<BuildTargetEventKind>,
        data: Option<serde_json::Value>,
    ) -> Self {
        Self { target, kind, data }
    }
    pub fn new_simple(target: BspBuildTargetIdentifier) -> Self {
        Self {
            target,
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BuildTargetEventKind {
    /// The build target is new [default].
    Created = 1,
    /// The build target has changed.
    Changed = 2,
    /// The build target has been deleted.
    Deleted = 3,
}

impl Default for BuildTargetEventKind {
    fn default() -> Self {
        Self::Created
    }
}
