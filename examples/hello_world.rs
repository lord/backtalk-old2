extern crate backtalk;
extern crate tokio_hyper as http;

use std::time::Duration;
use std::thread;

fn main() {
    http::Server::new()
        .serve(|| backtalk::service::server())
        .unwrap();

    thread::sleep(Duration::from_secs(1_000_000));
}
