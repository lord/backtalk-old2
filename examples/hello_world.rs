extern crate backtalk;
extern crate futures;
extern crate tokio_hyper as http;

use std::collections::HashMap;
use backtalk::resource::{Resource, Reply, ListReply};
use backtalk::params::Params;
use std::time::Duration;
use std::thread;
use futures::{Future, finished};

struct MyResource;

impl Resource for MyResource {
  type Object = HashMap<String, String>;
  type Error = String;
  type Id = i32;

  fn find(&self, _: &Params) -> ListReply<Self> {
    let mut map = HashMap::new();
    map.insert("test".to_string(), "blah".to_string());
    let v = vec![map];
    self.resp(v)
  }

  fn get(&self, _: &Self::Id, _: &Params) -> Reply<Self> {
    let mut map = HashMap::new();
    map.insert("test".to_string(), "blah".to_string());
    finished::<Self::Object, Self::Error>(map).boxed()
  }

  fn create(&self, _: &Self::Object, p: &Params) -> Reply<Self> {
    self.get(&1, p)
  }
  fn update(&self, _: &Self::Id, _: &Self::Object, p: &Params) -> Reply<Self> {
    self.get(&1, p)
  }
  fn patch(&self, _: &Self::Id, _: &Self::Object, p: &Params) -> Reply<Self> {
    self.get(&1, p)
  }
  fn remove(&self, _: &Self::Id, p: &Params) -> Reply<Self> {
    self.get(&1, p)
  }
}

fn main() {
    http::Server::new()
        .serve(|| MyResource.into_server())
        .unwrap();

    thread::sleep(Duration::from_secs(1_000_000));
}
