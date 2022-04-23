use crate::TaskDataKind;
use crate::TaskId;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct TaskStart {
    /// Unique id of the task with optional reference to parent task id
    pub task_id: super::TaskId,

    /// Timestamp of when the event started in milliseconds since Epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<u32>,

    /// Message describing the task.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub message: String,

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

impl TaskStart {
    pub fn new(task_id: impl Into<TaskId>) -> Self {
        Self {
            task_id: task_id.into(),
            ..Self::default()
        }
    }
}
