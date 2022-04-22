use serde::{Deserialize, Serialize};
/// Free-form string tags to categorize or label this build target.
/// For example, can be used by the client to:
///  - customize how the target should be translated into the client's project model.
///  - group together different but related targets in the user interface.
///  - display icons or colors in the user interface.
///  Pre-defined tags are listed in `BuildTargetTag` but clients and servers
///  are free to define new tags for custom purposes.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BuildTargetTag {
    /// Target contains re-usable functionality for downstream targets. May have
    /// any combination of capabilities.
    Libary,
    /// Target contains source code for producing any kind of application, may
    /// have but does not require the `canRun` capability.
    Application,
    /// Target contains source code for testing purposes, may have but does not
    /// require the `canTest` capability.
    Test,
    /// Target contains source code for integration testing purposes, may have
    /// but does not require the `canTest` capability. The difference between
    /// "test" and "integration-test" is that integration tests traditionally run
    /// slower compared to normal tests and require more computing resources to
    /// execute.
    IntegrationTest,
    /// Target contains source code to measure performance of a program, may have
    /// but does not require the `canRun` build target capability.
    Benchmark,
    /// Target should be ignored by IDEs.
    NoIDE,
    // Actions on the target such as build and test should only be invoked manually
    // and explicitly. For example, triggering a build on all targets in the workspace
    // should by default not include this target.
    Manual,
    /// Custom build target tag
    Custom(String),
}

impl Default for BuildTargetTag {
    fn default() -> Self {
        Self::NoIDE
    }
}

impl BuildTargetTag {
    pub fn custom<S: Into<String>>(value: S) -> Self {
        Self::Custom(value.into())
    }
}
