use std::path::{Path, PathBuf};

use crate::ClientCapabilities;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
/// Like the language server protocol, the initialize request is sent as the first request from the
/// client to the server. If the server receives a request or notification before the initialize
/// request it should act as follows:
///
/// - For a request the response should be an error with code: -32002. The message can be picked by
/// the server.
///
/// - Notifications should be dropped, except for the exit notification. This will allow the exit
/// of a server without an initialize request.
///
/// Until the server has responded to the initialize request with an [`InitializeBuildResult`], the
/// client must not send any additional requests or notifications to the server.
pub struct InitializeBuildParams {
    /// Name of the client
    display_name: String,
    /// The version of the client
    version: String,
    /// The BSP version that the client speaks
    bsp_version: String,
    /// The rootUri of the workspace
    root_uri: PathBuf,
    /// The capabilities of the client
    capabilities: ClientCapabilities,
    /// Additional metadata about the client
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

impl InitializeBuildParams {
    pub fn new<P: Into<PathBuf>>(
        display_name: String,
        version: String,
        bsp_version: String,
        root_uri: P,
        capabilities: ClientCapabilities,
        data: Option<Value>,
    ) -> Self {
        Self {
            display_name,
            version,
            bsp_version,
            root_uri: root_uri.into(),
            capabilities,
            data,
        }
    }

    pub fn new_simple<P: Into<PathBuf>>(
        display_name: String,
        version: String,
        bsp_version: String,
        root_uri: P,
        capabilities: ClientCapabilities,
    ) -> Self {
        Self {
            display_name,
            version,
            bsp_version,
            root_uri: root_uri.into(),
            capabilities,
            data: None,
        }
    }

    /// Set the bsp initialize build's data.
    pub fn set_data(&mut self, data: Option<Value>) {
        self.data = data;
    }

    /// Get a reference to the bsp initialize build params's data.
    pub fn data(&self) -> Option<&Value> {
        self.data.as_ref()
    }

    /// Get a reference to the bsp initialize build params's capabilities.
    pub fn capabilities(&self) -> &ClientCapabilities {
        &self.capabilities
    }

    /// Set the bsp initialize build params's capabilities.
    pub fn set_capabilities(&mut self, capabilities: ClientCapabilities) {
        self.capabilities = capabilities;
    }

    /// Get a reference to the bsp initialize build params's root uri.
    pub fn root_uri(&self) -> &Path {
        self.root_uri.as_ref()
    }

    /// Set the bsp initialize build params's root uri.
    pub fn set_root_uri<P: Into<PathBuf>>(&mut self, root_uri: P) {
        self.root_uri = root_uri.into();
    }

    /// Get a reference to the bsp initialize build params's bsp version.
    pub fn bsp_version(&self) -> &str {
        self.bsp_version.as_ref()
    }

    /// Set the bsp initialize build params's bsp version.
    pub fn set_bsp_version(&mut self, bsp_version: String) {
        self.bsp_version = bsp_version;
    }

    /// Get a reference to the bsp initialize build params's version.
    pub fn version(&self) -> &str {
        self.version.as_ref()
    }

    /// Set the bsp initialize build params's version.
    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    /// Get a reference to the bsp initialize build params's display name.
    pub fn display_name(&self) -> &str {
        self.display_name.as_ref()
    }

    /// Set the bsp initialize build params's display name.
    pub fn set_display_name(&mut self, display_name: String) {
        self.display_name = display_name;
    }
}
