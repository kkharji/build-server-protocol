use crate::providers::*;
use serde::{Deserialize, Serialize};

/// Server Capabilities
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerCapabilities {
    /// The languages the server supports compilation
    /// via method "buildTarget/compile".
    #[serde(skip_serializing_if = "Option::is_none")]
    compile_provider: Option<CompileProvider>,

    /// The languages the server supports test execution
    /// via method "buildTarget/test"
    #[serde(skip_serializing_if = "Option::is_none")]
    test_provider: Option<TestProvider>,

    /// The languages the server supports run
    /// via method "buildTarget/run"
    #[serde(skip_serializing_if = "Option::is_none")]
    run_provider: Option<RunProvider>,

    /// The languages the server supports debugging
    /// via method "debugSession/start"
    #[serde(skip_serializing_if = "Option::is_none")]
    debug_provider: Option<DebugProvider>,

    /// The server can provide a list of targets that contain a
    /// single text document
    /// via the method "buildTarget/inverseSources"
    #[serde(skip_serializing_if = "Option::is_none")]
    inverse_sources_provider: Option<bool>,

    /// The server provides sources for library dependencies
    /// via method "buildTarget/dependencySources"
    #[serde(skip_serializing_if = "Option::is_none")]
    dependency_sources_provider: Option<bool>,

    /// The server cam provide a list of dependency modules (libraries with meta information)
    /// via method "buildTarget/dependencyModules"
    #[serde(skip_serializing_if = "Option::is_none")]
    dependency_modules_provider: Option<bool>,

    /// The server provides all the resource dependencies
    /// via method "buildTarget/resources"
    #[serde(skip_serializing_if = "Option::is_none")]
    resources_provider: Option<bool>,

    /// Reloading the build state through workspace/reload is supported
    #[serde(skip_serializing_if = "Option::is_none")]
    can_reload: Option<bool>,

    /// The server sends notifications to the client on build
    /// target change events via "buildTarget/didChange"
    #[serde(skip_serializing_if = "Option::is_none")]
    build_target_changed_provider: Option<bool>,
}

impl ServerCapabilities {
    /// Set the bsp server capabilities's compile provider.
    pub fn set_compile_provider(&mut self, compile_provider: Option<CompileProvider>) {
        self.compile_provider = compile_provider;
    }

    /// Get a reference to the bsp server capabilities's compile provider.
    pub fn compile_provider(&self) -> Option<&CompileProvider> {
        self.compile_provider.as_ref()
    }

    /// Set the bsp server capabilities's test provider.
    pub fn set_test_provider(&mut self, test_provider: Option<TestProvider>) {
        self.test_provider = test_provider;
    }

    /// Get a reference to the bsp server capabilities's test provider.
    pub fn test_provider(&self) -> Option<&TestProvider> {
        self.test_provider.as_ref()
    }

    /// Set the bsp server capabilities's run provider.
    pub fn set_run_provider(&mut self, run_provider: Option<RunProvider>) {
        self.run_provider = run_provider;
    }

    /// Get a reference to the bsp server capabilities's run provider.
    pub fn run_provider(&self) -> Option<&RunProvider> {
        self.run_provider.as_ref()
    }

    /// Set the bsp server capabilities's debug provider.
    pub fn set_debug_provider(&mut self, debug_provider: Option<DebugProvider>) {
        self.debug_provider = debug_provider;
    }

    /// Get a reference to the bsp server capabilities's debug provider.
    pub fn debug_provider(&self) -> Option<&DebugProvider> {
        self.debug_provider.as_ref()
    }

    /// Set the bsp server capabilities's inverse sources provider.
    pub fn set_inverse_sources_provider(&mut self, inverse_sources_provider: Option<bool>) {
        self.inverse_sources_provider = inverse_sources_provider;
    }

    /// Get the bsp server capabilities's inverse sources provider.
    pub fn inverse_sources_provider(&self) -> Option<bool> {
        self.inverse_sources_provider
    }

    /// Set the bsp server capabilities's dependency sources provider.
    pub fn set_dependency_sources_provider(&mut self, dependency_sources_provider: Option<bool>) {
        self.dependency_sources_provider = dependency_sources_provider;
    }

    /// Get the bsp server capabilities's dependency sources provider.
    pub fn dependency_sources_provider(&self) -> Option<bool> {
        self.dependency_sources_provider
    }

    /// Set the bsp server capabilities's dependency modules provider.
    pub fn set_dependency_modules_provider(&mut self, dependency_modules_provider: Option<bool>) {
        self.dependency_modules_provider = dependency_modules_provider;
    }

    /// Get the bsp server capabilities's dependency modules provider.
    pub fn dependency_modules_provider(&self) -> Option<bool> {
        self.dependency_modules_provider
    }

    /// Set the bsp server capabilities's resources provider.
    pub fn set_resources_provider(&mut self, resources_provider: Option<bool>) {
        self.resources_provider = resources_provider;
    }

    /// Get the bsp server capabilities's resources provider.
    pub fn resources_provider(&self) -> Option<bool> {
        self.resources_provider
    }

    /// Set the bsp server capabilities's can reload.
    pub fn set_can_reload(&mut self, can_reload: Option<bool>) {
        self.can_reload = can_reload;
    }

    /// Get the bsp server capabilities's can reload.
    pub fn can_reload(&self) -> Option<bool> {
        self.can_reload
    }

    /// Set the bsp server capabilities's build target changed provider.
    pub fn set_build_target_changed_provider(
        &mut self,
        build_target_changed_provider: Option<bool>,
    ) {
        self.build_target_changed_provider = build_target_changed_provider;
    }

    /// Get the bsp server capabilities's build target changed provider.
    pub fn build_target_changed_provider(&self) -> Option<bool> {
        self.build_target_changed_provider
    }
}
