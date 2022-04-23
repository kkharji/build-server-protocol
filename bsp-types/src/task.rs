//! The BSP server can inform the client on the execution state of any task in the build tool. The
//! execution of some tasks, such as compilation or tests, must always be reported by the server.
//!
//! The server may also send additional task notifications for actions not covered by the protocol,
//! such as resolution or packaging. BSP clients can then display this information to their users
//! at their discretion.
//!
//! When beginning a task, the server may send build/taskStart, intermediate updates may be sent in
//! build/taskProgress.
//!
//! If a build/taskStart notification has been sent, the server must send build/taskFinish on
//! completion of the same task. Conversely, a build/taskFinish notification must always be sent
//! after a build/taskStart with the same taskId was sent.
//!
//! build/taskStart, build/taskProgress and build/taskFinish notifications for the same task must
//! use the same taskId.
//!
//! Tasks that are spawned by another task should reference the originating task's taskId in their
//! own taskId's parent field. Tasks spawned directly by a request should reference the request's
//! originId parent.
//!

mod finish;
mod id;
mod kind;
mod progress;
mod start;
mod status;

pub use finish::TaskFinish;
pub use id::TaskId;
pub use kind::TaskDataKind;
pub use progress::TaskProgress;
pub use start::TaskStart;
pub use status::TaskStatus;
