extern crate warp;

use warp::filters::log;

pub fn log_request(info: log::Info) {
    // Use a log macro, or slog, or println, or whatever!
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
