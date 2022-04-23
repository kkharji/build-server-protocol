use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{TaskDataKind, TaskId};

/// After a taskStart and before taskFinish for a taskId, the server may send any number of progress notifications.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct TaskProgress {
    /// Unique id of the task with optional reference to parent task id
    pub task_id: TaskId,

    /// Timestamp of when the progress event was generated in milliseconds since Epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<u32>,

    /// Message describing the task progress.
    ///  * Information about the state of the task at the time the event is sent.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub message: String,

    /// If known, total amount of work units in this task.
    pub total: Option<u32>,

    /// If known, completed amount of work units in this task.
    pub progress: Option<u32>,

    /// Name of a work unit. For example, "files" or "tests". May be empty.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub unit: String,

    /// Kind of data to expect in the `data` field. If this field is not set, the kind of data is not specified.
    ///  * Kind names for specific tasks like compile, test, etc are specified in the protocol.
    #[serde(skip_serializing_if = "TaskDataKind::is_none")]
    pub data_kind: TaskDataKind,

    /// Optional metadata about the task.
    ///  * Objects for specific tasks like compile, test, etc are specified in the protocol.
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    pub data: Value,
}
