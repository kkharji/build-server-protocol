use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
/// The Task Id allows clients to uniquely identify a BSP task and establish a client-parent
/// relationship with another task id.
pub struct TaskId {
    /// A unique identifier
    id: String,

    /// The parent task ids, if any. A non-empty parents field means
    ///  * this task is a sub-task of every parent task id. The child-parent
    ///  * relationship of tasks makes it possible to render tasks in
    ///  * a tree-like user interface or inspect what caused a certain task
    ///  * execution.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    parents: Vec<String>,
}

impl From<String> for TaskId {
    fn from(id: String) -> Self {
        Self::new_simple(id)
    }
}

impl From<&str> for TaskId {
    fn from(id: &str) -> Self {
        Self::new_simple(id.into())
    }
}

impl TaskId {
    pub fn new_simple(id: String) -> Self {
        Self {
            id,
            parents: Default::default(),
        }
    }

    pub fn new(id: String, parents: Vec<String>) -> Self {
        Self { id, parents }
    }

    pub fn add_parent(&mut self, value: String) {
        self.parents.push(value)
    }
}
