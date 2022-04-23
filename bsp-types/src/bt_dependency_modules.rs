use super::BuildTargetIdentifier;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The build target dependency modules request is sent from the client to the server to query for
/// the libraries of build target dependencies that are external to the workspace including meta
/// information about library and their sources. It's an extended version of buildTarget/sources.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuildTargetDependencyModule {
    /// The build targets to clean.
    targets: Vec<BuildTargetIdentifier>,
}

impl BuildTargetDependencyModule {
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

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuildTargetDependencyModuleResult {
    items: Vec<BTDependencyModulesItem>,
}

impl BuildTargetDependencyModuleResult {
    pub fn new(items: Vec<BTDependencyModulesItem>) -> Self {
        Self { items }
    }

    /// Get a reference to the bsp btdependency modules result's items.
    pub fn items(&self) -> &[BTDependencyModulesItem] {
        self.items.as_ref()
    }

    /// Get a mutable reference to the bsp btdependency modules result's items.
    pub fn items_mut(&mut self) -> &mut Vec<BTDependencyModulesItem> {
        &mut self.items
    }
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BTDependencyModulesItem {
    target: BuildTargetIdentifier,
    modules: Vec<BTDependencyModule>,
}

impl BTDependencyModulesItem {
    pub fn new(target: BuildTargetIdentifier, modules: Vec<BTDependencyModule>) -> Self {
        Self { target, modules }
    }

    /// Get a reference to the bsp btdependency modules item's target.
    pub fn target(&self) -> &BuildTargetIdentifier {
        &self.target
    }

    /// Set the bsp btdependency modules item's modules.
    pub fn set_modules(&mut self, modules: Vec<BTDependencyModule>) {
        self.modules = modules;
    }

    /// Get a reference to the bsp btdependency modules item's modules.
    pub fn modules(&self) -> &[BTDependencyModule] {
        self.modules.as_ref()
    }
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BTDependencyModule {
    /// Module name
    name: String,

    /// Module version
    version: String,

    /// Kind of data to expect in the `data` field. If this field is not set, the kind of data is not specified.
    data_kind: Option<String>,

    /// Language-specific metadata about this module.
    ///    * See https://github.com/build-server-protocol/build-server-protocol/blob/master/bsp4j/src/main/java/ch/epfl/scala/bsp4j/MavenExtension.xtend
    data: Option<Value>,
}

impl BTDependencyModule {
    pub fn new(
        name: String,
        version: String,
        data_kind: Option<String>,
        data: Option<Value>,
    ) -> Self {
        Self {
            name,
            version,
            data_kind,
            data,
        }
    }
    pub fn new_simple(name: String, version: String) -> Self {
        Self {
            name,
            version,
            data_kind: None,
            data: None,
        }
    }

    /// Get a reference to the bsp btdependency module's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Set the bsp btdependency module's name.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Get a reference to the bsp btdependency module's version.
    pub fn version(&self) -> &str {
        self.version.as_ref()
    }

    /// Set the bsp btdependency module's version.
    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    /// Get a reference to the bsp btdependency module's data kind.
    pub fn data_kind(&self) -> Option<&String> {
        self.data_kind.as_ref()
    }

    /// Set the bsp btdependency module's data kind.
    pub fn set_data_kind(&mut self, data_kind: Option<String>) {
        self.data_kind = data_kind;
    }
}
