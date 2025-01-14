//! Unix Domain Socket
//!
//! ```sh
//! curl --unix-socket /tmp/viz.sock http://localhost/
//! ```
#![deny(warnings)]
#![allow(clippy::unused_async)]

#[cfg(unix)]
#[tokio::main]
async fn main() -> viz::Result<()> {
    use std::sync::Arc;

    use tokio::net::UnixListener;
    use viz::{get, serve, IntoHandler, Result, Router, Tree};

    async fn index() -> Result<&'static str> {
        Ok("Hello world!")
    }

    let path = "/tmp/viz.sock";
    println!("listening on http://{path}");

    let listener = UnixListener::bind(path)?;

    let app = Router::new().route("/", get(index.into_handler()));
    let tree = Arc::new(Tree::from(app));

    loop {
        let (stream, _) = listener.accept().await?;
        let tree = tree.clone();
        tokio::task::spawn(async move {
            if let Err(err) = serve(stream, tree, None).await {
                eprintln!("Error while serving HTTP connection: {err}");
            }
        });
    }
}

#[cfg(not(unix))]
#[tokio::main]
async fn main() {
    panic!("Must run under Unix-like platform!");
}
