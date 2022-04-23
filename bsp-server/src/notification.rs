use std::fmt;

use bsp_types::{
    BuildTargetDidChange, LogMessage, PublishDiagnostics, ShowMessage, TaskFinish, TaskProgress,
    TaskStart,
};
use serde::{
    de::{Error as DeError, MapAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize,
};
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum Notification {
    Exit,
    Initialized,
    ShowMessage(ShowMessage),
    LogMessage(LogMessage),
    PublishDiagnostics(PublishDiagnostics),
    TaskStart(TaskStart),
    TaskFinish(TaskFinish),
    TaskProgress(TaskProgress),
    BuildTargetDidChange(BuildTargetDidChange),
    Custom(String, Value),
}

impl Notification {
    pub fn method(&self) -> &str {
        use Notification::*;
        match self {
            Exit => "build/exit",
            Initialized => "build/initialized",
            ShowMessage(_) => "build/showMessage",
            LogMessage(_) => "build/logMessage",
            PublishDiagnostics(_) => "build/publishDiagnostics",
            TaskStart(_) => "build/taskStart",
            TaskFinish(_) => "build/taskFinish",
            TaskProgress(_) => "build/taskProgressing",
            BuildTargetDidChange(_) => "buildTarget/didChange",
            Custom(m, _) => m,
        }
    }
}

macro_rules! from_type {
    ($p:ident) => {
        impl From<$p> for Notification {
            fn from(msg: $p) -> Self {
                Self::$p(msg)
            }
        }
    };
}

impl From<(String, Value)> for Notification {
    fn from(v: (String, Value)) -> Self {
        Self::Custom(v.0, v.1)
    }
}

impl From<&str> for Notification {
    fn from(msg: &str) -> Self {
        match msg {
            "build/exit" => Self::Exit,
            "build/initialized" => Self::Initialized,
            _ => panic!("Only exit and initialized supported."),
        }
    }
}

from_type!(ShowMessage);
from_type!(LogMessage);
from_type!(PublishDiagnostics);
from_type!(TaskStart);
from_type!(TaskFinish);
from_type!(TaskProgress);
from_type!(BuildTargetDidChange);

impl Serialize for Notification {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut obj = s.serialize_struct("Notification", 2)?;
        obj.serialize_field("method", self.method())?;

        use Notification::*;
        match self {
            // TODO: Should it set value to None?
            Exit | Self::Initialized => {}
            ShowMessage(m) => obj.serialize_field("params", m)?,
            LogMessage(m) => obj.serialize_field("params", m)?,
            PublishDiagnostics(m) => obj.serialize_field("params", m)?,
            TaskStart(m) => obj.serialize_field("params", m)?,
            TaskFinish(m) => obj.serialize_field("params", m)?,
            TaskProgress(m) => obj.serialize_field("params", m)?,
            BuildTargetDidChange(m) => obj.serialize_field("params", m)?,
            Custom(_, m) => obj.serialize_field("params", m)?,
        };

        obj.end()
    }
}

#[cfg(test)]
mod se {
    use super::*;
    #[test]
    fn initialized() {
        let value = &Notification::Initialized;
        let result = serde_json::to_string(value).unwrap();
        assert_eq!(result, "{\"method\":\"build/initialized\"}");
    }

    #[test]
    fn exit() {
        let value = &Notification::Exit;
        let result = serde_json::to_string(value).unwrap();
        assert_eq!(result, "{\"method\":\"build/exit\"}");
    }

    #[test]
    fn show_message() {
        let value = &Notification::TaskStart(TaskStart::new("some_id"));
        let result = serde_json::to_string(value).unwrap();
        assert_eq!(
            result,
            "{\"method\":\"build/taskStart\",\"params\":{\"taskId\":{\"id\":\"some_id\"}}}"
        );
    }

    #[test]
    fn custom() {
        let value = &Notification::Custom("custom".into(), Value::Null);
        let result = serde_json::to_string(value).unwrap();
        assert_eq!(result, "{\"method\":\"custom\",\"params\":null}");
    }
}

impl<'de> Deserialize<'de> for Notification {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &'static [&'static str] = &["params", "method"];
        enum Field {
            Method,
            Params,
            Other,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("method and params")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: DeError,
                    {
                        match value {
                            "method" => Ok(Field::Method),
                            "params" => Ok(Field::Params),
                            _ => Ok(Field::Other),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct NotificationVisitor;

        impl<'de> Visitor<'de> for NotificationVisitor {
            type Value = Notification;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Notification")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Notification, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut params: Option<serde_json::Value> = None; // this is just lazy
                let mut method: Option<String> = None; // required for json! to work
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Params => {
                            if params.is_some() {
                                return Err(DeError::duplicate_field("params"));
                            }
                            params = Some(map.next_value()?);
                        }
                        Field::Method => {
                            if method.is_some() {
                                return Err(DeError::duplicate_field("method"));
                            }
                            method = Some(map.next_value()?);
                        }
                        _ => (),
                    }
                }

                let method = method.ok_or_else(|| DeError::missing_field("method"))?;
                let params = match params {
                    Some(v) => v,
                    None => {
                        if &method != "build/exit" && &method != "build/initialized" {
                            return Err(DeError::missing_field("params"));
                        }
                        serde_json::Value::Null
                    }
                };

                fn de<'a, T: Deserialize<'a>, E: DeError>(p: serde_json::Value) -> Result<T, E> {
                    T::deserialize(p).map_err(DeError::custom)
                }

                use Notification::*;
                Ok(match method.as_str() {
                    "build/exit" => Exit,
                    "build/initialized" => Initialized,
                    "build/showMessage" => ShowMessage(de(params)?),
                    "build/logMessage" => LogMessage(de(params)?),
                    "build/publishDiagnostics" => PublishDiagnostics(de(params)?),
                    "build/taskStart" => TaskStart(de(params)?),
                    "build/taskFinish" => TaskFinish(de(params)?),
                    "build/taskProgressing" => TaskProgress(de(params)?),
                    "buildTarget/didChange" => BuildTargetDidChange(de(params)?),
                    method => Custom(method.into(), params),
                })
            }
        }

        deserializer.deserialize_struct("Notification", FIELDS, NotificationVisitor)
    }
}

#[cfg(test)]
mod de {
    use super::*;
    #[test]
    fn initialized_without_params() {
        let value = "{\"method\":\"build/initialized\"}";
        let msg = serde_json::from_str(value).unwrap();
        assert!(matches!(msg, Notification::Initialized));
    }

    #[test]
    fn initialized_with_params() {
        let value = serde_json::json!({
             "jsonrpc": "2.0",
             "method":"build/initialized"
        });
        let result = serde_json::from_value(value).unwrap();
        assert!(matches!(result, Notification::Initialized));
    }

    #[test]
    fn show_message() {
        let value = "{\"method\":\"build/taskStart\",\"params\":{\"taskId\":{\"id\":\"some_id\"}}}";
        let result = serde_json::from_str::<Notification>(value).unwrap();
        assert!(matches!(result, Notification::TaskStart(TaskStart { .. })));
    }
}
