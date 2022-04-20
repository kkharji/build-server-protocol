use crate::BuildTargetIdentifier;
use serde::{Deserialize, Serialize};
/// The build target resources request is sent from the client to the server to query for the list
/// of resources of a given list of build targets.
///
/// A resource is a data dependency required to be present in the runtime classpath when a build
/// target is run or executed. The server communicates during the initialize handshake whether this
/// method is supported or not.
///
/// This request can be used by a client to highlight the resources in a project view, for example.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BTResourcesParams {
    targets: Vec<BuildTargetIdentifier>,
}

impl BTResourcesParams {
    pub fn new(targets: Vec<BuildTargetIdentifier>) -> Self {
        Self { targets }
    }

    /// Get a reference to the resources params's targets.
    pub fn targets(&self) -> &[BuildTargetIdentifier] {
        self.targets.as_ref()
    }

    /// Set the resources params's targets.
    pub fn set_targets(&mut self, targets: Vec<BuildTargetIdentifier>) {
        self.targets = targets;
    }

    /// Get a mutable reference to the resources params's targets.
    pub fn targets_mut(&mut self) -> &mut Vec<BuildTargetIdentifier> {
        &mut self.targets
    }
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BTResourcesResult {
    items: Vec<BTResourcesItem>,
}

impl BTResourcesResult {
    pub fn new(items: Vec<BTResourcesItem>) -> Self {
        Self { items }
    }

    /// Get a reference to the bsp resources result's items.
    pub fn items(&self) -> &[BTResourcesItem] {
        self.items.as_ref()
    }

    /// Get a mutable reference to the bsp resources result's items.
    pub fn items_mut(&mut self) -> &mut Vec<BTResourcesItem> {
        &mut self.items
    }
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BTResourcesItem {
    target: BuildTargetIdentifier,
    /// List of resource files.
    resources: Vec<String>,
}

impl BTResourcesItem {
    pub fn new(target: BuildTargetIdentifier, resources: Vec<String>) -> Self {
        Self { target, resources }
    }

    /// Set the bsp resources item's target.
    pub fn set_target(&mut self, target: BuildTargetIdentifier) {
        self.target = target;
    }

    /// Get a reference to the bsp resources item's target.
    pub fn target(&self) -> &BuildTargetIdentifier {
        &self.target
    }

    /// Set the bsp resources item's resources.
    pub fn set_resources(&mut self, resources: Vec<String>) {
        self.resources = resources;
    }

    /// Get a reference to the bsp resources item's resources.
    pub fn resources(&self) -> &[String] {
        self.resources.as_ref()
    }
}
