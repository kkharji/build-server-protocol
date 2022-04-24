#[derive(Clone, Debug, serde_repr::Deserialize_repr, serde_repr::Serialize_repr)]
#[repr(u16)]
pub enum MessageType {
    /// An error message.
    Error = 1,
    /// A warning message.
    Warning = 2,
    /// An information message.
    Info = 3,
    /// A log message. (default)
    Log = 4,
}

impl Default for MessageType {
    fn default() -> Self {
        Self::Log
    }
}
