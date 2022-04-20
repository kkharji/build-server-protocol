use crate::ServerCapabilities;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Initialize Build response result
#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeBuildResult {
    /// Name of the server
    display_name: String,
    /// The version of the server
    version: String,
    /// The BSP version that the server speaks
    bsp_version: String,
    /// The capabilities of the build server
    capabilities: ServerCapabilities,
    /// Optional metadata about the server
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

impl InitializeBuildResult {
    pub fn new(
        display_name: String,
        version: String,
        bsp_version: String,
        capabilities: ServerCapabilities,
        data: Option<Value>,
    ) -> Self {
        Self {
            display_name,
            version,
            bsp_version,
            capabilities,
            data,
        }
    }

    pub fn new_simple(
        display_name: String,
        version: String,
        bsp_version: String,
        capabilities: ServerCapabilities,
    ) -> Self {
        Self {
            display_name,
            version,
            bsp_version,
            capabilities,
            data: None,
        }
    }

    /// Set the bsp initialize build result's bsp version.
    pub fn set_bsp_version(&mut self, bsp_version: String) {
        self.bsp_version = bsp_version;
    }

    /// Get a reference to the bsp initialize build result's bsp version.
    pub fn bsp_version(&self) -> &str {
        self.bsp_version.as_ref()
    }

    /// Set the bsp initialize build result's version.
    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    /// Get a reference to the bsp initialize build result's version.
    pub fn version(&self) -> &str {
        self.version.as_ref()
    }

    /// Set the bsp initialize build result's display name.
    pub fn set_display_name(&mut self, display_name: String) {
        self.display_name = display_name;
    }

    /// Get a reference to the bsp initialize build result's display name.
    pub fn display_name(&self) -> &str {
        self.display_name.as_ref()
    }

    /// Set the bsp initialize build result's capabilities.
    pub fn set_capabilities(&mut self, capabilities: ServerCapabilities) {
        self.capabilities = capabilities;
    }

    /// Get a reference to the bsp initialize build result's capabilities.
    pub fn capabilities(&self) -> &ServerCapabilities {
        &self.capabilities
    }

    /// Set the bsp initialize build result's data.
    pub fn set_data(&mut self, data: Option<Value>) {
        self.data = data;
    }

    /// Get a reference to the bsp initialize build result's data.
    pub fn data(&self) -> Option<&Value> {
        self.data.as_ref()
    }

    /// Get a mutable reference to the bsp initialize build result's data.
    pub fn data_mut(&mut self) -> &mut Option<Value> {
        &mut self.data
    }
}
