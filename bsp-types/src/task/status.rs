use core::fmt;

use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TaskStatus {
    /// Execution was successful.
    Ok = 1,
    /// Execution failed.
    Error = 2,
    /// Execution was cancelled.
    Cancelled = 3,
}

impl TaskStatus {
    /// Returns `true` if the task status is [`Ok`].
    ///
    /// [`Ok`]: TaskStatus::Ok
    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Ok)
    }

    /// Returns `true` if the task status is [`Error`].
    ///
    /// [`Error`]: TaskStatus::Error
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    /// Returns `true` if the task status is [`Cancelled`].
    ///
    /// [`Cancelled`]: TaskStatus::Cancelled
    pub fn is_cancelled(&self) -> bool {
        matches!(self, Self::Cancelled)
    }
}

impl Default for TaskStatus {
    fn default() -> Self {
        Self::Ok
    }
}

impl fmt::Debug for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Ok => f.write_str("TaskStatus::Ok"),
            TaskStatus::Error => f.write_str("TaskStatus::Error"),
            TaskStatus::Cancelled => f.write_str("TaskStatus::Cancelled"),
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Ok => f.write_str("Ok"),
            TaskStatus::Error => f.write_str("Error"),
            TaskStatus::Cancelled => f.write_str("Cancelled"),
        }
    }
}
