extern crate warp;

use std::convert::Infallible;
use tokio::sync::mpsc;
use warp::{filters::log, Filter};

pub fn log_request(info: log::Info) {
    // 127.0.0.1 - peter [9/Feb/2017:10:34:12 -0700] "GET /sample-image.png HTTP/2" 200 1479
    eprintln!(
        "{} \"{} {}\" {} {}",
        info.remote_addr().unwrap().ip(),
        info.method(),
        info.path(),
        info.status().as_u16(),
        info.elapsed().as_millis()
    );
}

#[derive(Clone)]
pub struct Context {
    pub tx: mpsc::Sender<String>,
}

impl Context {
    pub fn new(tx: mpsc::Sender<String>) -> Context {
        Context { tx }
    }
}

pub async fn listen(mut rx: mpsc::Receiver<String>) {
    while let Some(v) = rx.recv().await {
        println!("[fubar-proxy] {}", v);
    }
}

pub fn with_context(ctx: Context) -> impl Filter<Extract = (Context,), Error = Infallible> + Clone {
    warp::any().map(move || ctx.clone())
}
