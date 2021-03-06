extern crate reqwest;
extern crate warp;

mod proxy;
mod telemetry;

use std::error;
use std::net;
use warp::{http, hyper::body, path, Filter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let (tx, rx) = tokio::sync::mpsc::channel(100);
    let client = proxy::Client::new();
    let ctx = telemetry::Context::new(tx);
    let proxy = warp::any();
    //.map(warp::reply)
    //.with(warp::log::custom(telemetry::log_request))
    //.map(|_| {})
    //.untuple_one();

    let proxy = proxy
        .and(telemetry::with_context(ctx))
        .and(proxy::with_client(client))
        .and(warp::method())
        .and(warp::path::full())
        .and(warp::body::bytes())
        .and(warp::header::headers_cloned())
        .and_then(handle);

    let proxy = proxy.with(warp::log::custom(telemetry::log_request));

    let listen_addr: net::SocketAddr = "0.0.0.0:3000".parse().unwrap();

    let l = tokio::spawn(telemetry::listen(rx));
    let s = tokio::spawn(warp::serve(proxy).run(listen_addr));

    tokio::try_join!(l, s).unwrap();
    //warp::serve(proxy).run(listen_addr).await;
    Ok(())
}

async fn handle(
    ctx: telemetry::Context,
    client: proxy::Client,
    method: http::Method,
    path: path::FullPath,
    body: body::Bytes,
    headers: http::HeaderMap,
) -> Result<impl warp::Reply, warp::Rejection> {
    let reply = client.handle(ctx, method, path, body, headers).await;
    Ok(warp::reply::with_header(
        reply,
        "X-Powered-By",
        "fubar-proxy/0.0.1",
    ))
}
