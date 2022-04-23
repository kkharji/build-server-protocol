use crate::{MessageType, TaskId};
use serde::{Deserialize, Serialize};

/// The show message notification is sent from a server to a client to ask the client to display a
/// particular message in the user interface.
///
/// A build/showMessage notification is similar to LSP's window/showMessage, except for a few
/// additions like id and originId.
///
/// The originId field helps clients know which request originated
/// a notification in case several requests are handled by the client at the same time. It will
/// only be populated if the client defined it in the request that triggered this notification.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShowMessage {
    /// The message type. See {@link MessageType}.
    #[serde(rename = "type")]
    pub typ: MessageType,

    /// The actual message.
    pub message: String,

    /// The task id if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<TaskId>,

    /// The request id that originated this notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
}

impl ShowMessage {
    pub fn new<S: Into<String>>(
        typ: MessageType,
        msg: S,
        task: Option<TaskId>,
        orid: Option<S>,
    ) -> Self {
        Self {
            typ,
            task: task.map(Into::into),
            origin_id: orid.map(Into::into),
            message: msg.into(),
        }
    }

    /// Send info message.
    pub fn info<S: Into<String>>(msg: S, task: Option<TaskId>, orid: Option<S>) -> Self {
        Self::new(MessageType::Info, msg, task, orid)
    }

    /// Send log message.
    pub fn log<S: Into<String>>(msg: S, task: Option<TaskId>, orid: Option<S>) -> Self {
        Self::new(MessageType::Log, msg, task, orid)
    }

    /// Send warn message.
    pub fn warn<S: Into<String>>(msg: S, task: Option<TaskId>, orid: Option<S>) -> Self {
        Self::new(MessageType::Warning, msg, task, orid)
    }

    /// Send error message.
    pub fn error<S: Into<String>>(msg: S, task: Option<TaskId>, orid: Option<S>) -> Self {
        Self::new(MessageType::Error, msg, task, orid)
    }
}
