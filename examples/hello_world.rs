extern crate backtalk;
extern crate futures;
extern crate tokio_hyper as http;

use std::collections::HashMap;
use backtalk::resource::{Resource, Reply, ListReply};
use std::time::Duration;
use std::thread;
use futures::{Future, finished};

struct MyResource;

impl Resource for MyResource {
  type Object = HashMap<String, String>;
  type Error = String;

  fn list(&self) -> ListReply<Self> {
    let mut map = HashMap::new();
    map.insert("test".to_string(), "blah".to_string());
    let v = vec![map];
    self.resp(v)
  }

  fn obj(&self) -> Reply<Self> {
    let mut map = HashMap::new();
    map.insert("test".to_string(), "blah".to_string());
    finished::<Self::Object, Self::Error>(map).boxed()
  }
}

fn main() {
    http::Server::new()
        .serve(|| MyResource.serve())
        .unwrap();

    thread::sleep(Duration::from_secs(1_000_000));
}
