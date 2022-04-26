//! A build server scaffold, exposing a synchronous crossbeam-channel based API.
//! This crate handles protocol handshaking and parsing messages, while you
//! control the message dispatch loop yourself.
//!
//! Run with `RUST_LOG=bsp_server=trace` to see all the messages.
//!
//! inspired by lsp-server
mod error;
mod io_thread;
mod message;
// mod req_queue;
mod request;
mod response;
mod transporter;

mod notification;
#[cfg(test)]
mod tests;
pub use bsp_types as types;
pub use error::{ErrorCode, ExtractError, ProtocolError};
pub use io_thread::IoThreads;
pub use message::Message;
pub use notification::Notification;
// pub use req_queue::{Incoming, Outgoing, ReqQueue};
pub use request::{Request, RequestId};
pub use response::{Response, ResponseError};
pub(crate) use transporter::Transporter;

use bsp_types::InitializeBuild;
use crossbeam_channel::{unbounded, Receiver, SendError, SendTimeoutError, Sender, TrySendError};
use serde::Serialize;
use std::io;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};

/// Connection is just a pair of channels of LSP messages.
pub struct Connection {
    pub sender: Sender<Message>,
    pub receiver: Receiver<Message>,
}

impl Connection {
    /// Create connection over standard in/standard out.
    ///
    /// Use this to create a real language server.
    pub fn stdio() -> (Connection, IoThreads) {
        let Transporter(sender, receiver, io_threads) = Transporter::stdio();
        (Connection { sender, receiver }, io_threads)
    }

    /// Open a connection over tcp.
    /// This call blocks until a connection is established.
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<(Connection, IoThreads)> {
        let stream = TcpStream::connect(addr)?;
        let Transporter(sender, receiver, io_threads) = Transporter::socket(stream);
        Ok((Connection { sender, receiver }, io_threads))
    }

    /// Listen for a connection over tcp.
    /// This call blocks until a connection is established.
    pub fn listen<A: ToSocketAddrs>(addr: A) -> io::Result<(Connection, IoThreads)> {
        let listener = TcpListener::bind(addr)?;
        let (stream, _) = listener.accept()?;
        let Transporter(sender, receiver, io_threads) = Transporter::socket(stream);
        Ok((Connection { sender, receiver }, io_threads))
    }

    /// Creates a pair of connected connections in memory for testing.
    pub fn memory() -> (Connection, Connection) {
        let ((s1, r1), (s2, r2)) = (unbounded(), unbounded());
        (
            Connection {
                sender: s1,
                receiver: r2,
            },
            Connection {
                sender: s2,
                receiver: r1,
            },
        )
    }

    /// Initialize the connection. Sends the server initialize response
    /// to the client and returns the serialized client capabilities
    /// on success. If more fine-grained initialization is required use
    /// `initialize_start`/`initialize_finish`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::error::Error;
    /// use build_server_protocol::types::InitializeBuildResult;
    /// use build_server_protocol::server::{Connection, Message, Request, RequestId, Response};
    ///
    /// fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    ///    // Create the transport
    ///    let (conn, io_threads) = Connection::stdio();
    ///
    ///    // Run the server
    ///    conn.initialize(|_params| {
    ///        Ok(InitializeBuildResult::new_simple(
    ///         "MyBuildServer",
    ///         "0.1",
    ///         "2.0",
    ///         ServerCapabilities::default()))
    ///    })?;
    ///
    ///    // ... Run main loop ...
    ///
    ///    Ok(())
    /// }
    /// ```
    #[tracing::instrument(skip_all)]
    pub fn initialize<V: Serialize>(
        &self,
        process: impl FnOnce(&InitializeBuild) -> V,
    ) -> Result<InitializeBuild, ProtocolError> {
        let (id, params) = self.initialize_start()?;
        self.initialize_finish(id, process(&params))?;
        Ok(params)
    }

    #[tracing::instrument(skip(self))]
    fn initialize_start(&self) -> Result<(RequestId, InitializeBuild), ProtocolError> {
        loop {
            match self.receiver.recv() {
                Ok(Message::Request(Request::InitializeBuild(id, params))) => {
                    return Ok((id, params));
                }
                // Respond to non-initialize requests with ServerNotInitialized
                Ok(Message::Request(req)) => {
                    let msg = format!("expected initialize request, got {:?}", req);
                    tracing::error!("{}", msg);
                    self.sender
                        .send(Response::server_not_initialized(req.id().clone(), msg).into())
                        .unwrap();
                }
                Ok(msg) => {
                    let msg = format!("expected initialize request, got {:?}", msg);
                    tracing::error!("{}", msg);
                    return Err(ProtocolError(msg));
                }
                Err(e) => {
                    let msg = format!("expected initialize request, got error: {}", e);
                    tracing::error!("{}", msg);
                    return Err(ProtocolError(msg));
                }
            };
        }
    }

    /// Finishes the initialization process by sending an `InitializeResult` to the client
    #[tracing::instrument(skip_all)]
    fn initialize_finish<V: Serialize>(
        &self,
        initialize_id: RequestId,
        initialize_result: V,
    ) -> Result<(), ProtocolError> {
        let resp = Response::ok(initialize_id, initialize_result);
        self.sender.send(resp.into()).unwrap();
        match &self.receiver.recv() {
            Ok(Message::Notification(Notification::Initialized)) => (),
            Ok(msg) => {
                let msg = format!("expected Message::Notification, got: {:?}", msg,);
                tracing::error!("{}", msg);
                return Err(ProtocolError(msg));
            }
            Err(e) => {
                let msg = format!("expected initialized notification, got error: {}", e,);
                tracing::error!("{}", msg);
                return Err(ProtocolError(msg));
            }
        }
        Ok(())
    }

    /// If `req` is `Shutdown`, respond to it and return `true`, otherwise return `false`
    pub fn handle_shutdown(&self, req: &Request) -> Result<bool, ProtocolError> {
        if let Request::Shutdown(id) = req {
            tracing::info!("processing shutdown server ...");
            let resp = Response::ok(id.clone(), ());
            let _ = self.sender.send(resp.into());
            match &self.receiver.recv_timeout(Duration::from_secs(30)) {
                Ok(Message::Notification(Notification::Exit)) => (),
                Ok(msg) => {
                    let msg = format!("unexpected message during shutdown: {:?}", msg);
                    tracing::error!("{}", msg);

                    return Err(ProtocolError(msg));
                }
                Err(e) => {
                    let msg = format!("unexpected error during shutdown: {}", e);
                    return Err(ProtocolError(msg));
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// delegates to self.sender
    pub fn send<T: Into<Message>>(&self, msg: T) -> Result<(), SendError<Message>> {
        self.sender.send(msg.into())
    }

    /// delegates to self.sender
    pub fn try_send<T: Into<Message>>(&self, msg: T) -> Result<(), TrySendError<Message>> {
        self.sender.try_send(msg.into())
    }

    /// delegates to self.sender
    pub fn send_timeout<T: Into<Message>>(
        &self,
        msg: T,
        timeout: Duration,
    ) -> Result<(), SendTimeoutError<Message>> {
        self.sender.send_timeout(msg.into(), timeout)
    }

    /// delegates to self.sender
    pub fn send_deadline<T: Into<Message>>(
        &self,
        msg: T,
        deadline: Instant,
    ) -> Result<(), SendTimeoutError<Message>> {
        self.sender.send_deadline(msg.into(), deadline)
    }
}
