use crate::BspBuildTargetIdentifier;
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
pub struct BspResourcesParams {
    targets: Vec<BspBuildTargetIdentifier>,
}

impl BspResourcesParams {
    pub fn new(targets: Vec<BspBuildTargetIdentifier>) -> Self {
        Self { targets }
    }

    /// Get a reference to the resources params's targets.
    pub fn targets(&self) -> &[BspBuildTargetIdentifier] {
        self.targets.as_ref()
    }

    /// Set the resources params's targets.
    pub fn set_targets(&mut self, targets: Vec<BspBuildTargetIdentifier>) {
        self.targets = targets;
    }

    /// Get a mutable reference to the resources params's targets.
    pub fn targets_mut(&mut self) -> &mut Vec<BspBuildTargetIdentifier> {
        &mut self.targets
    }
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BspResourcesResult {
    items: Vec<BspResourcesItem>,
}

impl BspResourcesResult {
    pub fn new(items: Vec<BspResourcesItem>) -> Self {
        Self { items }
    }

    /// Get a reference to the bsp resources result's items.
    pub fn items(&self) -> &[BspResourcesItem] {
        self.items.as_ref()
    }

    /// Get a mutable reference to the bsp resources result's items.
    pub fn items_mut(&mut self) -> &mut Vec<BspResourcesItem> {
        &mut self.items
    }
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BspResourcesItem {
    target: BspBuildTargetIdentifier,
    /// List of resource files.
    resources: Vec<String>,
}

impl BspResourcesItem {
    pub fn new(target: BspBuildTargetIdentifier, resources: Vec<String>) -> Self {
        Self { target, resources }
    }

    /// Set the bsp resources item's target.
    pub fn set_target(&mut self, target: BspBuildTargetIdentifier) {
        self.target = target;
    }

    /// Get a reference to the bsp resources item's target.
    pub fn target(&self) -> &BspBuildTargetIdentifier {
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
