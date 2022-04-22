use crate::{Message, Notification, Request, RequestId};

#[test]
fn shutdown_with_explicit_null() {
    let text = "{\"jsonrpc\": \"2.0\",\"id\": 3,\"method\": \"build/shutdown\", \"params\": null }";
    let msg: Message = serde_json::from_str(text).unwrap();

    assert!(
        matches!(msg, Message::Request(req) if req.id == 3.into() && req.method == "build/shutdown")
    );
}

#[test]
fn shutdown_with_no_params() {
    let text = "{\"jsonrpc\": \"2.0\",\"id\": 3,\"method\": \"build/shutdown\"}";
    let msg: Message = serde_json::from_str(text).unwrap();

    assert!(
        matches!(msg, Message::Request(req) if req.id == 3.into() && req.method == "build/shutdown")
    );
}

#[test]
fn notification_with_explicit_null() {
    let text = "{\"jsonrpc\": \"2.0\",\"method\": \"build/exit\", \"params\": null }";
    let msg: Message = serde_json::from_str(text).unwrap();

    assert!(matches!(msg, Message::Notification(not) if not.method == "build/exit"));
}

#[test]
fn notification_with_no_params() {
    let text = "{\"jsonrpc\": \"2.0\",\"method\": \"build/exit\"}";
    let msg: Message = serde_json::from_str(text).unwrap();

    assert!(matches!(msg, Message::Notification(not) if not.method == "build/exit"));
}

#[test]
fn serialize_request_with_null_params() {
    let msg = Message::Request(Request {
        id: RequestId::from(3),
        method: "build/shutdown".into(),
        params: serde_json::Value::Null,
    });
    let serialized = serde_json::to_string(&msg).unwrap();

    assert_eq!("{\"id\":3,\"method\":\"build/shutdown\"}", serialized);
}

#[test]
fn serialize_notification_with_null_params() {
    let msg = Message::Notification(Notification {
        method: "build/exit".into(),
        params: serde_json::Value::Null,
    });
    let serialized = serde_json::to_string(&msg).unwrap();

    assert_eq!("{\"method\":\"build/exit\"}", serialized);
}
