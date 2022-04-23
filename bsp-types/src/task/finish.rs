use crate::TaskDataKind;
use crate::TaskId;
use crate::TaskStatus;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct TaskFinish {
    /// Unique id of the task with optional reference to parent task id
    pub task_id: super::TaskId,

    /** Timestamp of the event in milliseconds. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<u32>,

    /// Message describing the task.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub message: String,

    /** Task completion status. */
    pub status: TaskStatus,

    /// Kind of data to expect in the `data` field. If this field is not set, the kind of data is
    /// not specified.
    /// * Kind names for specific tasks like compile, test, etc are specified in the protocol.
    #[serde(skip_serializing_if = "TaskDataKind::is_none")]
    pub data_kind: TaskDataKind,

    /// Optional metadata about the task.
    /// * Objects for specific tasks like compile, test, etc are specified in the protocol.
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    pub data: Value,
}

impl TaskFinish {
    pub fn new(task_id: TaskId) -> Self {
        Self {
            task_id,
            ..Self::default()
        }
    }
}
