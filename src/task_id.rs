use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The Task Id allows clients to uniquely identify a BSP task and establish a client-parent
/// relationship with another task id.
pub struct BspTaskId {
    /// A unique identifier
    id: String,

    /// The parent task ids, if any. A non-empty parents field means
    ///  * this task is a sub-task of every parent task id. The child-parent
    ///  * relationship of tasks makes it possible to render tasks in
    ///  * a tree-like user interface or inspect what caused a certain task
    ///  * execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    parents: Option<Vec<String>>,
}

impl BspTaskId {
    pub fn new_simple(id: String) -> Self {
        Self { id, parents: None }
    }

    pub fn new(id: String, parents: Option<Vec<String>>) -> Self {
        Self { id, parents }
    }
}
