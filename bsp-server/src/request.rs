mod id;

use crate::Message;
use std::fmt;

use bsp_types::*;
pub use id::*;

use serde::{
    de::{Error as DeError, MapAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize,
};

use serde_json::Value;

#[derive(Clone)]
pub enum Request {
    /// Client->Server: Initialize Server
    InitializeBuild(RequestId, InitializeBuild),
    /// Client->Server: Shutdown server
    Shutdown(RequestId),
    /// Client->Server: Get a list of all available build targets in the workspace.
    WorkspaceBuildTargets(RequestId),
    /// Client->Server: Reload the build configuration.
    WorkspaceReload(RequestId),
    /// Client->Server: Get libraries of build target dependencies that are external to the
    /// workspace including meta information about library and their sources. It's an extended
    /// version of buildTarget/sources
    BuildTargetDependencyModules(RequestId, BuildTargetDependencyModules),
    /// Client->Server: Debug build target(s)
    DebugSessionStart(RequestId, DebugSessionStart),
    /// Client->Server: Get text documents and directories that belong to a build target.
    BuildTargetSources(RequestId, BuildTargetSources),
    /// Client->Server: Get build targets containing a text document.
    TextDocumentInverseSources(RequestId, TextDocumentInverseSources),
    /// Client->Server: Get sources of build target dependencies that are external to the
    /// workspace.
    BuildTargetDependencySources(RequestId, BuildTargetDependencySources),
    /// Client->Server: Get list of resources of a given list of build targets.
    BuildTargetResources(RequestId, BuildTargetResources),
    /// Client->Server: Run a build target
    BuildTargetRun(RequestId, BuildTargetRun),
    /// Client->Server: Run a compile target
    BuildTargetCompile(RequestId, BuildTargetCompile),
    /// Client->Server: Run a test target
    BuildTargetTest(RequestId, BuildTargetTest),
    /// Client->Server: reset any state associated with a given build target
    BuildTargetCleanCache(RequestId, BuildTargetCleanCache),
    /// Server-Client: Ask Client to show a message
    ShowMessage(RequestId, ShowMessage),
    /// Server-Client: Ask Client to log a message
    LogMessage(RequestId, LogMessage),
    /// Any custom message not yet supported in the crate or custom
    Custom(RequestId, &'static str, Value),
}

impl Request {
    pub fn method(&self) -> &'static str {
        use Request::*;
        match self {
            InitializeBuild(_, _) => "build/initialize",
            Shutdown(_) => "build/shutdown",
            WorkspaceBuildTargets(_) => "workspace/buildTargets",
            WorkspaceReload(_) => "workspace/reload",
            BuildTargetDependencyModules(_, _) => "buildTarget/dependencyModules",
            DebugSessionStart(_, _) => "debugSession/start",
            BuildTargetSources(_, _) => "buildTarget/sources",
            TextDocumentInverseSources(_, _) => "textDocument/inverseSources",
            BuildTargetDependencySources(_, _) => "buildTarget/dependencySources",
            BuildTargetResources(_, _) => "buildTarget/resources",
            BuildTargetRun(_, _) => "buildTarget/run",
            BuildTargetCompile(_, _) => "buildTarget/compile",
            BuildTargetTest(_, _) => "buildTarget/test",
            BuildTargetCleanCache(_, _) => "buildTarget/cleanCache",
            Custom(_, m, _) => m,
            ShowMessage(_, _) => "build/showMessage",
            LogMessage(_, _) => "build/logMessage",
        }
    }

    pub fn id(&self) -> &RequestId {
        use Request::*;
        match self {
            InitializeBuild(id, _)
            | Shutdown(id)
            | WorkspaceBuildTargets(id)
            | WorkspaceReload(id)
            | BuildTargetDependencyModules(id, _)
            | DebugSessionStart(id, _)
            | BuildTargetSources(id, _)
            | TextDocumentInverseSources(id, _)
            | BuildTargetDependencySources(id, _)
            | BuildTargetResources(id, _)
            | BuildTargetRun(id, _)
            | BuildTargetCompile(id, _)
            | BuildTargetTest(id, _)
            | BuildTargetCleanCache(id, _)
            | LogMessage(id, _)
            | ShowMessage(id, _)
            | Custom(id, _, _) => id,
        }
    }
}

impl From<Request> for Message {
    fn from(request: Request) -> Message {
        Message::Request(request)
    }
}

impl From<(RequestId, &'static str, Value)> for Request {
    fn from(v: (RequestId, &'static str, Value)) -> Self {
        Self::Custom(v.0.into(), v.1, v.2)
    }
}

impl From<(RequestId, &'static str, Value)> for Message {
    fn from(v: (RequestId, &'static str, Value)) -> Self {
        Self::Request((v.0.into(), v.1, v.2).into())
    }
}

macro_rules! convertible {
    ($p:ident) => {
        impl From<(RequestId, $p)> for Request {
            fn from(v: (RequestId, $p)) -> Self {
                Self::$p(v.0, v.1)
            }
        }

        impl From<(RequestId, $p)> for Message {
            fn from(v: (RequestId, $p)) -> Self {
                Self::Request(crate::Request::$p(v.0, v.1))
            }
        }
    };
}

convertible!(BuildTargetCleanCache);
convertible!(BuildTargetCompile);
convertible!(BuildTargetDependencyModules);
convertible!(BuildTargetDependencySources);
convertible!(BuildTargetResources);
convertible!(BuildTargetRun);
convertible!(BuildTargetSources);
convertible!(BuildTargetTest);
convertible!(DebugSessionStart);
convertible!(InitializeBuild);
convertible!(TextDocumentInverseSources);

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn format(
            f: &mut fmt::Formatter<'_>,
            id: &RequestId,
            value: impl fmt::Debug,
        ) -> fmt::Result {
            fmt::Display::fmt(&id, f)?;
            f.write_str(", ")?;
            value.fmt(f)
        }
        match self {
            Request::InitializeBuild(id, value) => format(f, id, value),
            Request::Shutdown(id) => format(f, id, "Shutdown"),
            Request::WorkspaceBuildTargets(id) => format(f, id, "WorkspaceBuildTargets"),
            Request::WorkspaceReload(id) => format(f, id, "WorkspaceReload"),
            Request::BuildTargetDependencyModules(id, value) => format(f, id, value),
            Request::DebugSessionStart(id, value) => format(f, id, value),
            Request::BuildTargetSources(id, value) => format(f, id, value),
            Request::TextDocumentInverseSources(id, value) => format(f, id, value),
            Request::BuildTargetDependencySources(id, value) => format(f, id, value),
            Request::BuildTargetResources(id, value) => format(f, id, value),
            Request::BuildTargetRun(id, value) => format(f, id, value),
            Request::BuildTargetCompile(id, value) => format(f, id, value),
            Request::BuildTargetTest(id, value) => format(f, id, value),
            Request::BuildTargetCleanCache(id, value) => format(f, id, value),
            Request::ShowMessage(id, value) => format(f, id, value),
            Request::LogMessage(id, value) => format(f, id, value),
            Request::Custom(id, method, value) => {
                fmt::Display::fmt(&id, f)?;
                f.write_str(", ")?;
                method.fmt(f)?;
                f.write_str(",\n")?;
                value.fmt(f)
            }
        }
    }
}

impl Serialize for Request {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let method = self.method();
        let mut obj = s.serialize_struct("Request", 2)?;

        use Request::*;
        match self {
            InitializeBuild(id, value) => {
                obj.serialize_field("id", id)?;
                obj.serialize_field("method", method)?;
                obj.serialize_field("params", value)?;
            }
            // TODO: Should it set value to None?
            Shutdown(id) | WorkspaceBuildTargets(id) | WorkspaceReload(id) => {
                obj.serialize_field("id", id)?;
                obj.serialize_field("method", method)?;
            }
            BuildTargetDependencyModules(id, value) => {
                obj.serialize_field("id", id)?;
                obj.serialize_field("method", method)?;
                obj.serialize_field("params", value)?;
            }
            DebugSessionStart(id, value) => {
                obj.serialize_field("id", id)?;
                obj.serialize_field("method", method)?;
                obj.serialize_field("params", value)?;
            }
            BuildTargetSources(id, value) => {
                obj.serialize_field("id", id)?;
                obj.serialize_field("method", method)?;
                obj.serialize_field("params", value)?;
            }
            TextDocumentInverseSources(id, value) => {
                obj.serialize_field("id", id)?;
                obj.serialize_field("method", method)?;
                obj.serialize_field("params", value)?;
            }
            BuildTargetDependencySources(id, value) => {
                obj.serialize_field("id", id)?;
                obj.serialize_field("method", method)?;
                obj.serialize_field("params", value)?;
            }
            BuildTargetResources(id, value) => {
                obj.serialize_field("id", id)?;
                obj.serialize_field("method", method)?;
                obj.serialize_field("params", value)?;
            }
            BuildTargetRun(id, value) => {
                obj.serialize_field("id", id)?;
                obj.serialize_field("method", method)?;
                obj.serialize_field("params", value)?;
            }
            BuildTargetCompile(id, value) => {
                obj.serialize_field("id", id)?;
                obj.serialize_field("method", method)?;
                obj.serialize_field("params", value)?;
            }
            BuildTargetTest(id, value) => {
                obj.serialize_field("id", id)?;
                obj.serialize_field("method", method)?;
                obj.serialize_field("params", value)?;
            }
            LogMessage(id, value) => {
                obj.serialize_field("id", id)?;
                obj.serialize_field("method", method)?;
                obj.serialize_field("params", value)?;
            }
            ShowMessage(id, value) => {
                obj.serialize_field("id", id)?;
                obj.serialize_field("method", method)?;
                obj.serialize_field("params", value)?;
            }
            BuildTargetCleanCache(id, value) => {
                obj.serialize_field("id", id)?;
                obj.serialize_field("method", method)?;
                obj.serialize_field("params", value)?;
            }
            Custom(id, _, value) => {
                obj.serialize_field("id", id)?;
                obj.serialize_field("method", method)?;
                if !value.is_null() {
                    obj.serialize_field("params", value)?;
                }
            }
        };
        obj.end()
    }
}

#[cfg(test)]
mod se {
    use serde_json::to_string;

    use super::*;
    #[test]
    fn initialize() {
        let mut params = InitializeBuild::default();
        params.set_display_name("MyName".into());

        let value = &Request::InitializeBuild(3.into(), params);
        let result = to_string(value).unwrap();
        assert_eq!(
            result,
            "{\"id\":3,\"method\":\"build/initialize\",\"params\":{\"displayName\":\"MyName\",\"capabilities\":{\"languageIds\":[]}}}"
        );
    }

    #[test]
    fn shutdown() {
        let value = &Request::Shutdown(3.into());
        let result = to_string(value).unwrap();
        assert_eq!(result, "{\"id\":3,\"method\":\"build/shutdown\"}");
    }

    #[test]
    fn debug_session_start() {
        let mut params = DebugSessionStart::default();
        params.set_data_kind("Some".into());

        let value = &Request::DebugSessionStart(3.into(), params);
        let result = to_string(value).unwrap();
        assert_eq!(result, "{\"id\":3,\"method\":\"debugSession/start\",\"params\":{\"targets\":[],\"dataKind\":\"Some\"}}");
    }

    #[test]
    fn custom() {
        let value = &Request::Custom(3.into(), "some/method", Value::Null);
        let result = to_string(value).unwrap();
        assert_eq!(result, "{\"id\":3,\"method\":\"some/method\"}");
    }
}

impl<'de> Deserialize<'de> for Request {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &'static [&'static str] = &["id", "method", "params"];
        enum Field {
            ID,
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
                            "id" => Ok(Field::ID),
                            "method" => Ok(Field::Method),
                            "params" => Ok(Field::Params),
                            _ => Ok(Field::Other),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct RequestVisitor;

        impl<'de> Visitor<'de> for RequestVisitor {
            type Value = Request;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Request")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Request, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut id: Option<Value> = None; //  not sure maybe string maybe i32
                let mut method: Option<String> = None; // required for json! to work
                let mut params: Option<Value> = None; // this is just lazy

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::ID => {
                            if id.is_some() {
                                return Err(DeError::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
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

                fn de<'a, T: Deserialize<'a>, E: DeError>(p: serde_json::Value) -> Result<T, E> {
                    T::deserialize(p).map_err(DeError::custom)
                }

                let method = method.ok_or_else(|| DeError::missing_field("method"))?;
                let id = de::<RequestId, _>(id.ok_or_else(|| DeError::missing_field("id"))?)?;
                let params = match params {
                    Some(v) => v,
                    None => {
                        if &method != "build/shutdown"
                            || &method != "workspace/buildTargets"
                            || &method != "workspace/reload"
                        {
                            return Err(DeError::missing_field("params"));
                        }
                        serde_json::Value::Null
                    }
                };

                Ok(match method.as_str() {
                    "build/initialize" => Request::InitializeBuild(id, de(params)?),
                    "build/shutdown" => Request::Shutdown(id),
                    "workspace/buildTargets" => Request::WorkspaceBuildTargets(id),
                    "workspace/reload" => Request::WorkspaceReload(id),
                    "buildTarget/dependencyModules" => {
                        Request::BuildTargetDependencyModules(id, de(params)?)
                    }
                    "debugSession/start" => Request::DebugSessionStart(id, de(params)?),
                    "buildTarget/sources" => Request::BuildTargetSources(id, de(params)?),
                    "textDocument/inverseSources" => {
                        Request::TextDocumentInverseSources(id, de(params)?)
                    }
                    "buildTarget/dependencySources" => {
                        Request::BuildTargetDependencySources(id, de(params)?)
                    }
                    "buildTarget/resources" => Request::BuildTargetResources(id, de(params)?),
                    "buildTarget/run" => Request::BuildTargetRun(id, de(params)?),
                    "buildTarget/compile" => Request::BuildTargetCompile(id, de(params)?),
                    "buildTarget/test" => Request::BuildTargetTest(id, de(params)?),
                    "buildTarget/cleanCache" => Request::BuildTargetCleanCache(id, de(params)?),
                    "build/logMessage" => Request::LogMessage(id, de(params)?),
                    "build/showMessage" => Request::ShowMessage(id, de(params)?),
                    _ => Request::Custom(id, Box::leak(method.into_boxed_str()), params),
                })
            }
        }

        deserializer.deserialize_struct("Request", FIELDS, RequestVisitor)
    }
}

#[cfg(test)]
mod de {
    use super::*;
    #[test]
    fn initialize() {
        let value = "{\"id\":3,\"method\":\"build/initialize\",\"params\":{\"displayName\":\"MyName\",\"capabilities\":{\"languageIds\":[]}}}";
        let msg = serde_json::from_str(value).unwrap();
        assert!(matches!(
            msg,
            Request::InitializeBuild(_, InitializeBuild { .. })
        ));
    }
}
