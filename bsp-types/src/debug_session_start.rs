use super::BuildTargetIdentifier;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The debug request is sent from the client to the server to debug build target(s). The server
/// launches a Microsoft DAP server and returns a connection URI for the client to interact with.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DebugSessionStart {
    /// A sequence of build targets affected by the debugging action.
    targets: Vec<BuildTargetIdentifier>,

    /// The kind of data to expect in the `data` field.
    /// TODO: Is DebugSessionStart dataKind == TaskDataKind?
    data_kind: String,

    /// Language-specific metadata for this execution.
    ///  * See https://github.com/build-server-protocol/build-server-protocol/blob/master/bsp4j/src/main/xtend-gen/ch/epfl/scala/bsp4j/ScalaMainClass.java
    data: Value,
}

impl DebugSessionStart {
    pub fn new(targets: Vec<BuildTargetIdentifier>, data_kind: String, data: Value) -> Self {
        Self {
            targets,
            data_kind,
            data,
        }
    }

    /// Get a reference to the debug session params's targets.
    pub fn targets(&self) -> &[BuildTargetIdentifier] {
        self.targets.as_ref()
    }

    /// Get a reference to the debug session params's data kind.
    pub fn data_kind(&self) -> &str {
        self.data_kind.as_ref()
    }

    /// Get a reference to the debug session params's data.
    pub fn data(&self) -> &Value {
        &self.data
    }

    /// Set the debug session params's targets.
    pub fn set_targets(&mut self, targets: Vec<BuildTargetIdentifier>) {
        self.targets = targets;
    }

    /// Set the debug session params's data kind.
    pub fn set_data_kind(&mut self, data_kind: String) {
        self.data_kind = data_kind;
    }

    /// Set the debug session params's data.
    pub fn set_data(&mut self, data: Value) {
        self.data = data;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DebugSessionStartResult {
    /** The Debug Adapter Protocol server's connection uri */
    uri: String,
}
