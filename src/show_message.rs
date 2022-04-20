use crate::{BTTaskId, MessageType};
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
#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowMessage {
    /// The message type. See {@link MessageType}.
    #[serde(rename = "type")]
    typ: MessageType,

    /// The task id if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<BTTaskId>,

    /// The request id that originated this notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,

    /// The actual message.
    pub message: String,
}

impl ShowMessage {
    pub fn new(
        typ: MessageType,
        task: Option<BTTaskId>,
        origin_id: Option<String>,
        message: String,
    ) -> Self {
        Self {
            typ,
            task,
            origin_id,
            message,
        }
    }
    pub fn new_simple(typ: MessageType, message: String) -> Self {
        Self {
            typ,
            message,
            ..Default::default()
        }
    }
}
