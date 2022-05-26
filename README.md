# Build Server Protocol

State: Working, Unstable
[Build Server Protocol](https://build-server-protocol.github.io/docs/specification.html) client, server and type definition in rust.
## Install 

```
bsp-server = "0.1.3"
# OR if you want just types
bsp-types = "0.1.3"
```

## Example 

```rust 
use anyhow::Result;
use bsp_server::{types::*, *};

fn main() -> Result<()> {
    install_tracing("/tmp/", "xcodebase-server.log", false)?;
    let (conn, io_threads) = Connection::stdio();

    tracing::info!("Started------------------------------");

    let params = conn.initialize(|params| crate::server::initialize(&params).expect("Initialize"))?;

    block(conn, params)?;

    io_threads.join()?;

    tracing::info!("Ended ------------------------------");

    Ok(())
}

fn block(conn: Connection, _initialize_params: InitializeBuild) -> Result<()> {
    for msg in &conn.receiver {
        match msg {
            Message::Request(req) => {
                use Request::*;
                match req {
                    Shutdown(_) => {
                        conn.handle_shutdown(&req)?;
                        return Ok(());
                    }
                    WorkspaceBuildTargets(id) => {
                        conn.send((id, WorkspaceBuildTargetsResult::default()))?;
                    }
                    BuildTargetSources(id, _) => {
                        conn.send((id, BuildTargetSourcesResult::default()))?;
                    }
                    _ => {
                        tracing::warn!("Unable to handle:\n\n{:#?}\n", req);
                        conn.send(Response::method_not_found(req.id().clone(), "".into()))?;
                    }
                };
            }
            Message::Response(_) => {}
            Message::Notification(_) => {}
        };
    }
    Ok(())
}
```
