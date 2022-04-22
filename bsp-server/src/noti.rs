use crate::ExtractError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Notification {
    pub method: String,
    #[serde(default = "serde_json::Value::default")]
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    pub params: serde_json::Value,
}

impl Notification {
    pub fn new(method: String, params: impl Serialize) -> Notification {
        Notification {
            method,
            params: serde_json::to_value(params).unwrap(),
        }
    }
    pub fn extract<P: DeserializeOwned>(
        self,
        method: &str,
    ) -> Result<P, ExtractError<Notification>> {
        if self.method == method {
            serde_json::from_value(self.params).map_err(|error| ExtractError::JsonError {
                method: self.method,
                error,
            })
        } else {
            Err(ExtractError::MethodMismatch(self))
        }
    }

    pub(crate) fn is_exit(&self) -> bool {
        self.method == "build/exit"
    }

    pub(crate) fn is_initialized(&self) -> bool {
        self.method == "build/initialized"
    }

    pub fn is_show_message(&self) -> bool {
        self.method == "build/showMessage"
    }

    pub fn is_log_message(&self) -> bool {
        self.method == "build/logMessage"
    }

    pub fn is_publish_diagnostics(&self) -> bool {
        self.method == "build/publishDiagnostics"
    }
}
