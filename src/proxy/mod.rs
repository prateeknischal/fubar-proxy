extern crate reqwest;

use crate::telemetry;
use reqwest::Url;
use std::convert::Infallible;
use warp::{http, hyper::body, path, Filter};

#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Client {
        Client {
            client: reqwest::Client::new(),
        }
    }
    pub async fn handle(
        &self,
        mut ctx: telemetry::Context,
        method: http::Method,
        path: path::FullPath,
        body: body::Bytes,
        headers: http::HeaderMap,
    ) -> impl warp::Reply {
        let client_path = format!("http://localhost:8080{}", path.as_str());
        let url = Url::parse(client_path.as_str()).unwrap();
        let mut res = self
            .client
            .request(method, url)
            .body(body)
            .headers(headers)
            .send()
            .await
            .unwrap();

        let mut reply = http::Response::builder().status(res.status());
        let headers = res.headers_mut();
        for hv in headers.iter() {
            reply = reply.header(hv.0, hv.1);
        }
        let content = res.text().await.unwrap();
        ctx.tx
            .send(format!("{} {}", client_path, "localhost"))
            .await
            .unwrap();
        reply.body(content).unwrap()
    }
}

// blog.logrocket.com/create-an-async-crud-web-service-in-rust-with-warp/
pub fn with_client(client: Client) -> impl Filter<Extract = (Client,), Error = Infallible> + Clone {
    warp::any().map(move || client.clone())
}
