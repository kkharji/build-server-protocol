use crate::{ErrorCode, RequestId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    // JSON RPC allows this to be null if it was impossible
    // to decode the request's id. Ignore this special case
    // and just die horribly.
    pub id: RequestId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponseError>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl Response {
    pub fn ok<R: Serialize>(id: RequestId, result: R) -> Response {
        Response {
            id,
            result: Some(serde_json::to_value(result).unwrap()),
            error: None,
        }
    }

    pub fn err(id: RequestId, code: i32, message: String) -> Response {
        let error = ResponseError {
            code,
            message,
            data: None,
        };
        Response {
            id,
            result: None,
            error: Some(error),
        }
    }

    pub fn parse_error(id: RequestId, message: String) -> Response {
        Self::err(id, ErrorCode::ParseError as i32 as i32, message)
    }

    pub fn server_not_initialized(id: RequestId, message: String) -> Response {
        Self::err(id, ErrorCode::ServerNotInitialized as i32, message)
    }

    pub fn invalid_request(id: RequestId, message: String) -> Response {
        Self::err(id, ErrorCode::InvalidRequest as i32 as i32, message)
    }

    pub fn method_not_found(id: RequestId, message: String) -> Response {
        Self::err(id, ErrorCode::MethodNotFound as i32 as i32, message)
    }

    pub fn invalid_params(id: RequestId, message: String) -> Response {
        Self::err(id, ErrorCode::InvalidParams as i32 as i32, message)
    }

    pub fn internal_error(id: RequestId, message: String) -> Response {
        Self::err(id, ErrorCode::InternalError as i32 as i32, message)
    }

    pub fn server_error_start(id: RequestId, message: String) -> Response {
        Self::err(id, ErrorCode::ServerErrorStart as i32 as i32, message)
    }

    pub fn server_error_end(id: RequestId, message: String) -> Response {
        Self::err(id, ErrorCode::ServerErrorEnd as i32 as i32, message)
    }

    pub fn request_canceled(id: RequestId, message: String) -> Response {
        Self::err(id, ErrorCode::RequestCanceled as i32 as i32, message)
    }

    pub fn content_modified(id: RequestId, message: String) -> Response {
        Self::err(id, ErrorCode::ContentModified as i32 as i32, message)
    }

    pub fn server_canceled(id: RequestId, message: String) -> Response {
        Self::err(id, ErrorCode::ServerCancelled as i32 as i32, message)
    }
}
