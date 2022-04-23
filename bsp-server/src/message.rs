use crate::Notification;
use crate::Request;
use crate::Response;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Message {
    Request(Request),
    Response(Response),
    Notification(Notification),
}

#[derive(Serialize)]
struct JsonRpc {
    jsonrpc: &'static str,
    #[serde(flatten)]
    msg: Message,
}

impl From<Request> for Message {
    fn from(request: Request) -> Message {
        Message::Request(request)
    }
}

impl From<Response> for Message {
    fn from(response: Response) -> Message {
        Message::Response(response)
    }
}

impl From<Notification> for Message {
    fn from(notification: Notification) -> Message {
        Message::Notification(notification)
    }
}

impl From<Message> for JsonRpc {
    fn from(msg: Message) -> Self {
        Self {
            jsonrpc: "2.0",
            msg,
        }
    }
}

impl Message {
    pub(crate) fn read(r: &mut dyn io::BufRead) -> io::Result<Option<Message>> {
        let text = match read_msg_text(r)? {
            Some(text) => text,
            None => return Ok(None),
        };
        let msg = serde_json::from_str(&text)?;
        tracing::debug!("Got <<<<<<<<<<<<<<<<<<<<<<<<<\n\n{:#?}\n", msg);
        Ok(Some(msg))
    }

    pub(crate) fn write(self, w: &mut dyn io::Write) -> io::Result<()> {
        tracing::debug!("Sent >>>>>>>>>>>>>>>>>>>>>>>>>\n\n{:#?}\n", self);
        let msg = serde_json::to_string(&JsonRpc::from(self))?;
        write!(w, "Content-Length: {}\r\n\r\n", msg.len())?;
        w.write_all(msg.as_bytes())?;
        w.flush()?;
        Ok(())
    }
}

fn invalid_data(error: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error)
}

macro_rules! invalid_data {
        ($($tt:tt)*) => (invalid_data(format!($($tt)*)))
}

fn read_msg_text(inp: &mut dyn io::BufRead) -> io::Result<Option<String>> {
    let mut size = None;
    let mut buf = String::new();
    loop {
        buf.clear();
        if inp.read_line(&mut buf)? == 0 {
            return Ok(None);
        }
        if !buf.ends_with("\r\n") {
            return Err(invalid_data!("malformed header: {:?}", buf));
        }
        let buf = &buf[..buf.len() - 2];
        if buf.is_empty() {
            break;
        }
        let mut parts = buf.splitn(2, ": ");
        let header_name = parts.next().unwrap();
        let header_value = parts
            .next()
            .ok_or_else(|| invalid_data!("malformed header: {:?}", buf))?;
        if header_name == "Content-Length" {
            size = Some(header_value.parse::<usize>().map_err(invalid_data)?);
        }
    }
    let size: usize = size.ok_or_else(|| invalid_data!("no Content-Length"))?;
    let mut buf = buf.into_bytes();
    buf.resize(size, 0);
    inp.read_exact(&mut buf)?;
    let buf = String::from_utf8(buf).map_err(invalid_data)?;

    Ok(Some(buf))
}
