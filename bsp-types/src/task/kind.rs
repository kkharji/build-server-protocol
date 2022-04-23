use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

#[derive(Debug, Clone)]
pub enum TaskDataKind {
    CompileTask,
    CompileReport,
    TestTask,
    TestReport,
    TestStart,
    TestFinish,
    Custom(String),
    None,
}

// TODO: Test might break!
impl Serialize for TaskDataKind {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        let s = match self {
            TaskDataKind::CompileTask => "compile-task",
            TaskDataKind::CompileReport => "compile-report",
            TaskDataKind::TestTask => "test-task",
            TaskDataKind::TestReport => "test-report",
            TaskDataKind::TestStart => "test-start",
            TaskDataKind::TestFinish => "test-finish",
            TaskDataKind::Custom(x) => x.as_str(),
            TaskDataKind::None => return ser.serialize_none(),
        };

        ser.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for TaskDataKind {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        let s = match <String>::deserialize(de) {
            Ok(v) => v,
            Err(_) => return Ok(Self::None),
        };
        let v = match s.as_str() {
            "compile-task" => Self::CompileTask,
            "compile-report" => Self::CompileReport,
            "test-task" => Self::TestTask,
            "test-report" => Self::TestReport,
            "test-start" => Self::TestStart,
            "test-finish" => Self::TestFinish,
            x => Self::Custom(x.to_owned()),
        };
        Ok(v)
    }
}

impl Default for TaskDataKind {
    fn default() -> Self {
        Self::None
    }
}

impl TaskDataKind {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}
