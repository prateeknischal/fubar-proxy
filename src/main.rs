extern crate reqwest;
extern crate warp;

mod proxy;

use std::error;
use std::net;
use warp::{http, hyper::body, path, Filter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    listen_and_serve().await;
    return Ok(());
}

async fn listen_and_serve() {
    let client = proxy::Client::new();
    let proxy = warp::any()
        .and(proxy::with_client(client))
        .and(warp::method())
        .and(warp::path::full())
        .and(warp::body::bytes())
        .and(warp::header::headers_cloned())
        .and_then(handle);

    let listen_addr: net::SocketAddr = "0.0.0.0:3000".parse().unwrap();
    warp::serve(proxy).run(listen_addr).await;
}

async fn handle(
    client: proxy::Client,
    method: http::Method,
    path: path::FullPath,
    body: body::Bytes,
    headers: http::HeaderMap,
) -> Result<impl warp::Reply, warp::Rejection> {
    let reply = client.handle(method, path, body, headers).await;
    Ok(warp::reply::with_header(
        reply,
        "X-Powered-By",
        "fubar-proxy v0.0.1",
    ))
}
