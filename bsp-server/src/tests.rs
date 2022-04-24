use crate::{Message, Request};

#[test]
fn shutdown_with_explicit_null() {
    let text = "{\"jsonrpc\": \"2.0\",\"id\": 3,\"method\": \"build/shutdown\", \"params\": null }";
    let msg: Message = serde_json::from_str(text).unwrap();

    assert!(matches!(msg, Message::Request(Request::Shutdown(id)) if id == 3.into()));
}

#[test]
#[ignore = "requires implementation of response or figuring out a way to ignore params in custom serde::de"]
fn shutdown_with_no_params() {
    let text = "{\"jsonrpc\": \"2.0\",\"id\": 3,\"method\": \"build/shutdown\"}";
    let msg: Message = serde_json::from_str(text).unwrap();
    dbg!(&msg);

    assert!(matches!(msg, Message::Request(Request::Shutdown(id)) if id == 3.into()));
}

#[test]
fn serialize_request_with_null_params() {
    let msg = Message::Request(Request::Shutdown(3.into()));
    let serialized = serde_json::to_string(&msg).unwrap();

    assert_eq!("{\"id\":3,\"method\":\"build/shutdown\"}", serialized);
}
