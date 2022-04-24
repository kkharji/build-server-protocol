# BSP-Server

State: Working, Unstable, feature releases might break 0.1

A build server scaffold, exposing a synchronous crossbeam-channel based API.
This crate handles protocol handshaking and parsing messages, while you
control the message dispatch loop yourself.

Run with `RUST_LOG=bsp_server=debug` to see all the messages.

inspired by lsp-server
