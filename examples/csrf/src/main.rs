#![deny(warnings)]
#![allow(clippy::unused_async)]

use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::net::TcpListener;

use viz::{
    middleware::{
        cookie,
        csrf::{self, CsrfToken},
        helper::CookieOptions,
    },
    serve, Method, Request, RequestExt, Result, Router, Tree,
};

async fn index(mut req: Request) -> Result<String> {
    Ok(req.extract::<CsrfToken>().await?.0)
}

async fn create(_req: Request) -> Result<&'static str> {
    Ok("CSRF Protection!")
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("listening on http://{addr}");

    let app = Router::new()
        .get("/", index)
        .post("/", create)
        .with(csrf::Config::new(
            csrf::Store::Cookie,
            [Method::GET, Method::HEAD, Method::OPTIONS, Method::TRACE].into(),
            CookieOptions::new("_csrf").max_age(Duration::from_secs(3600 * 24)),
            csrf::secret,
            csrf::generate,
            csrf::verify,
        ))
        .with(cookie::Config::default());
    let tree = Arc::new(Tree::from(app));

    loop {
        let (stream, addr) = listener.accept().await?;
        let tree = tree.clone();
        tokio::task::spawn(async move {
            if let Err(err) = serve(stream, tree, Some(addr)).await {
                eprintln!("Error while serving HTTP connection: {err}");
            }
        });
    }
}
