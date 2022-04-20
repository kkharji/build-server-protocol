use crate::BuildTargetCapabilities;
use crate::BuildTargetIdentifier;
use crate::BuildTargetTag;
use crate::Language;
use serde::{Deserialize, Serialize};

/// The workspace build targets request is sent from the client to the server to
/// ask for the list of all available build targets in the workspace.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WorkspaceBuildTargetsResult {
    pub targets: Vec<BuildTarget>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildTarget {
    /// The target’s unique identifier
    pub id: BuildTargetIdentifier,

    /// A human readable name for this target.
    /// May be presented in the user interface.
    /// Should be unique if possible.
    /// The id.uri is used if None.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// The directory where this target belongs to. Multiple build targets are
    /// allowed to map to the same base directory, and a build target is not
    /// required to have a base directory. A base directory does not determine the
    /// sources of a target, see "buildTarget/sources".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_directory: Option<String>,

    /// Free-form string tags to categorize or label this build target.
    /// For example, can be used by the client to:
    /// - customize how the target should be translated into the client's project
    ///   model.
    /// - group together different but related targets in the user interface.
    /// - display icons or colors in the user interface.
    /// Pre-defined tags are listed in `BuildTargetTag` but clients and servers
    /// are free to define new tags for custom purposes.
    pub tags: Vec<BuildTargetTag>,

    /// The capabilities of this build target.
    pub capabilities: BuildTargetCapabilities,

    /// The set of languages that this target contains.
    /// The ID string for each language is defined in the LSP.
    pub language_ids: Vec<Language>,

    /// The direct upstream build target dependencies of this build target
    pub dependencies: Vec<BuildTargetIdentifier>,
}

impl BuildTarget {
    pub fn new(
        id: BuildTargetIdentifier,
        display_name: Option<String>,
        base_directory: Option<String>,
        tags: Vec<BuildTargetTag>,
        capabilities: BuildTargetCapabilities,
        language_ids: Vec<Language>,
        dependencies: Vec<BuildTargetIdentifier>,
    ) -> Self {
        Self {
            id,
            display_name,
            base_directory,
            tags,
            capabilities,
            language_ids,
            dependencies,
        }
    }
    pub fn new_simple(
        id: BuildTargetIdentifier,
        tags: Vec<BuildTargetTag>,
        capabilities: BuildTargetCapabilities,
        language_ids: Vec<Language>,
        dependencies: Vec<BuildTargetIdentifier>,
    ) -> Self {
        Self {
            id,
            tags,
            capabilities,
            language_ids,
            dependencies,
            ..Default::default()
        }
    }
}

impl WorkspaceBuildTargetsResult {
    pub fn new(targets: Vec<BuildTarget>) -> Self {
        Self { targets }
    }
}

impl From<BuildTarget> for WorkspaceBuildTargetsResult {
    fn from(target: BuildTarget) -> Self {
        Self {
            targets: vec![target],
        }
    }
}
