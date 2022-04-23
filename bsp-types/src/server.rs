#![allow(unused_variables)]
use super::*;
use anyhow::Result;
use serde_json::Value;

pub trait BuildServer {
    /// Invoked when client sends server "build/initialize"
    ///
    /// The initialize request is sent as the first request from the client to the server. If the
    /// server receives a request or notification before the initialize request it should act as
    /// follows:
    ///
    /// - For a request the response should be an error with code: -32002. The message can be
    /// picked by the server.
    ///
    /// - Notifications should be dropped, except for the exit notification. This will allow the
    /// exit of a server without an initialize request.
    ///
    /// Until the server has responded to the initialize request with an [`InitializeBuildResult`],
    /// the client must not send any additional requests or notifications to the server.
    // #[rpc(name = "build/initialize")]
    fn initialize(&self, params: InitializeBuildParams) -> Result<InitializeBuildResult>;

    /// Invoked when client sends server "build/initialized"
    ///
    /// A notification is sent from the client to the server after the client received the result
    /// of the initialize request but before the client is sending any other request or
    /// notification to the server. The server can use the initialized notification for example to
    /// initialize intensive computation such as dependency resolution or compilation. The
    /// initialized notification may only be sent once.
    // #[rpc(name = "build/initialized")]
    fn on_initializtion(&self) {}

    /// Invoked when client sends server "build/shutdown"
    ///
    /// The shutdown build request is sent from the client to the server. It asks the server to
    /// shut down, but to not exit (otherwise the response might not be delivered correctly to the
    /// client). There is a separate exit notification that asks the server to exit.
    // #[rpc(name = "build/shutdown")]
    fn build_shutdown(&self) -> Result<Option<Value>> {
        Ok(None)
    }

    /// Invoked when client sends server "build/exit"
    ///
    /// A notification to ask the server to exit its process. The server should exit with success
    /// code 0 if the shutdown request has been received before;
    /// otherwise with error code 1.
    // #[rpc(name = "build/exit")]
    fn on_build_exit(&self) {}

    /// Invoked when client sends server "workspace/buildTargets"
    ///
    /// The workspace build targets request is sent from the client to the server to ask for the
    /// list of all available build targets in the workspace.
    // #[rpc(name = "workspace/buildTargets")]
    fn workspace_bts(&self) -> Result<WorkspaceBuildTargetsResult> {
        Ok(WorkspaceBuildTargetsResult::default())
    }

    /// Invoked when client sends server "workspace/reload"
    ///
    // The reload request is sent from the client to instruct the build server to reload the build
    // configuration. This request should be supported by build tools that keep their state in memory.
    // If the reload request returns with an error, it's expected that other requests respond with the
    // previously known "good" state.
    // #[rpc(name = "workspace/reload")]
    fn workspace_reload(&self) -> Result<Option<Value>> {
        Ok(None)
    }

    /// Invoked when client sends server "buildTarget/dependencyModules"
    ///
    /// The build target dependency modules request is sent from the client to the server to query for the
    /// libraries of build target dependencies that are external to the workspace including meta
    /// information about library and their sources. It's an extended version of buildTarget/sources.
    // #[rpc(name = "buildTarget/dependencyModules")]
    fn bt_dependency_modules(
        &self,
        params: BuildTargetDependencyModule,
    ) -> Result<BuildTargetDependencyModuleResult> {
        todo!()
        // Err(Error::method_not_found())
    }

    /// Invoked when client sends server "buildTarget/dependencyModules"
    ///
    /// The debug request is sent from the client to the server to debug build target(s). The server
    /// launches a Microsoft DAP server and returns a connection URI for the client to interact with.
    // #[rpc(name = "debugSession/start")]
    fn debug_session_start(&self, params: DebugSessionStart) -> Result<DebugSessionStartResult> {
        todo!()
        // Err(Error::method_not_found())
    }

    /// Invoked when client sends server "buildTarget/sources"
    ///
    /// The build target sources request is sent from the client to the server to
    /// query for the list of text documents and directories that are belong to a
    /// build target. The sources response must not include sources that are
    /// external to the workspace.
    // #[rpc(name = "buildTarget/sources")]
    fn bt_sources(&self, params: BuildTargetSources) -> Result<BuildTargetSourcesResult> {
        Ok(BuildTargetSourcesResult::default())
    }

    /// Invoked when client sends server "buildTarget/inverseSources"
    // #[rpc(name = "buildTarget/sources")]
    fn bt_inverse_sources(
        &self,
        params: BuildTargetInverseSources,
    ) -> Result<BuildTargetInverseSourcesResult> {
        Ok(BuildTargetInverseSourcesResult::default())
    }

    /// Invoked when client sends server "buildTarget/dependencySources"
    ///
    /// The inverse sources request is sent from the client to the server to query for the list of
    /// build targets containing a text document. The server communicates during the initialize
    /// handshake whether this method is supported or not. This request can be viewed as the inverse of
    /// buildTarget/sources, except it only works for text documents and not directories.
    // #[rpc(name = "buildTarget/dependencySources")]
    fn bt_dependency_sources(
        &self,
        params: BuildTargetDependencySources,
    ) -> Result<BuildTargetDependencySourcesResult> {
        Ok(BuildTargetDependencySourcesResult::default())
    }

    /// Invoked when client sends server "buildTarget/resources"
    ///
    /// The build target resources request is sent from the client to the server to query for the list
    /// of resources of a given list of build targets.
    ///
    /// A resource is a data dependency required to be present in the runtime classpath when a build
    /// target is run or executed. The server communicates during the initialize handshake whether this
    /// method is supported or not.
    ///
    /// This request can be used by a client to highlight the resources in a project view, for example.
    // #[rpc(name = "buildTarget/resources")]
    fn bt_resources(&self, params: BuildTargetResources) -> Result<BuildTargetResourcesResult> {
        Ok(BuildTargetResourcesResult::default())
    }

    /// Invoked when client sends server "buildTarget/run"
    ///
    /// The run request is sent from the client to the server to run a build target. The server
    /// communicates during the initialize handshake whether this method is supported or not.
    // #[rpc(name = "buildTarget/run")]
    fn bt_run(&self, params: BuildTargetParams) -> Result<BuildTargetResult> {
        todo!()
        // Err(Error::method_not_found())
    }

    /// Invoked when client sends server "buildTarget/compile"
    ///
    /// The run request is sent from the client to the server to run a build target. The server
    /// communicates during the initialize handshake whether this method is supported or not.
    // #[rpc(name = "buildTarget/compile")]
    fn bt_compile(&self, params: BuildTargetCompile) -> Result<BuildTargetResult> {
        todo!()
        // Err(Error::method_not_found())
    }

    /// Invoked when client sends server "buildTarget/test"
    ///
    /// The test build target request is sent from the client to the server to test the given list of
    /// build targets. The server communicates during the initialize handshake whether this method is
    /// supported or not.
    // #[rpc(name = "buildTarget/test")]
    fn bt_test(&self, params: BuildTargetTest) -> Result<BuildTargetTestResult> {
        todo!()
        // Err(Error::method_not_found())
    }

    /// Invoked when client sends server "buildTarget/cleanCache"
    ///
    /// The clean cache request is sent from the client to the server to reset any state associated with
    /// a given build target. The state can live either in the build tool or in the file system.
    ///
    /// The build tool defines the exact semantics of the clean cache request:
    ///
    /// Stateless build tools are free to ignore the request and respond with a successful response.
    /// Stateful build tools must ensure that invoking compilation on a target that has been cleaned
    /// results in a full compilation.
    // #[rpc(name = "buildTarget/cleanCache")]
    fn bt_clean_cache(&self, params: BuildTargetCleanCache) -> Result<BuildTargetCleanCacheResult> {
        todo!()
        // Err(Error::method_not_found())
    }
}
