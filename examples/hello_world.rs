extern crate backtalk;
extern crate futures;
extern crate tokio_hyper as http;

use backtalk::resource::Resource;
use std::time::Duration;
use std::thread;
use futures::{Future, BoxFuture, finished};

struct MyResource;

impl Resource for MyResource {
  type Object = String;
  type Error = String;

  fn obj(&self) -> BoxFuture<Self::Object, Self::Error> {
    finished::<String, String>("meow".to_string()).boxed()
  }
}

fn main() {
    http::Server::new()
        .serve(|| MyResource.serve())
        .unwrap();

    thread::sleep(Duration::from_secs(1_000_000));
}
