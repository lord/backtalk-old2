extern crate backtalk;
extern crate tokio_hyper as http;

use backtalk::service::Resource;
use std::time::Duration;
use std::thread;

struct MyResource;

impl Resource for MyResource {
  fn text(&self) -> String {
    "from resource".to_string()
  }
}

fn main() {
    http::Server::new()
        .serve(|| MyResource{}.serve())
        .unwrap();

    thread::sleep(Duration::from_secs(1_000_000));
}
