use serde::{Deserialize, Serialize};

/// BuildTarget Capabilities
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BspBuildTargetCapabilities {
    /// This target can be compiled by the BSP server.
    can_compile: bool,

    /// This target can be tested by the BSP server.
    can_test: bool,

    /// This target can be run by the BSP server.
    can_run: bool,

    // This target can be debugged by the BSP server.
    can_debug: bool,
}

impl BspBuildTargetCapabilities {
    pub fn new(can_compile: bool, can_test: bool, can_run: bool, can_debug: bool) -> Self {
        Self {
            can_compile,
            can_test,
            can_run,
            can_debug,
        }
    }

    /// Set the bsp build target capabilities's can compile.
    pub fn set_can_compile(&mut self, can_compile: bool) {
        self.can_compile = can_compile;
    }

    /// Get the bsp build target capabilities's can compile.
    pub fn can_compile(&self) -> bool {
        self.can_compile
    }

    /// Set the bsp build target capabilities's can test.
    pub fn set_can_test(&mut self, can_test: bool) {
        self.can_test = can_test;
    }

    /// Get the bsp build target capabilities's can test.
    pub fn can_test(&self) -> bool {
        self.can_test
    }

    /// Set the bsp build target capabilities's can run.
    pub fn set_can_run(&mut self, can_run: bool) {
        self.can_run = can_run;
    }

    /// Get the bsp build target capabilities's can run.
    pub fn can_run(&self) -> bool {
        self.can_run
    }

    /// Set the bsp build target capabilities's can debug.
    pub fn set_can_debug(&mut self, can_debug: bool) {
        self.can_debug = can_debug;
    }

    /// Get the bsp build target capabilities's can debug.
    pub fn can_debug(&self) -> bool {
        self.can_debug
    }
}
