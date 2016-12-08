extern crate backtalk;
extern crate futures;
extern crate hyper;

use hyper::server::Server;
use std::collections::HashMap;
use backtalk::{Resource, Reply, ListReply, APIServer, Params, Router};
use futures::{Future, finished};

struct MyResource;

impl Resource for MyResource {
  type Object = HashMap<String, String>;
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
    finished(map).boxed()
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
  let server = Server::http(&"127.0.0.1:1337".parse().unwrap()).unwrap();
  let (listening, server) = server.standalone(|| {
    let mut router = Router::new();
    router.add_resource("myresource", MyResource{});
    Ok(APIServer::new(router))
  }).unwrap();
  println!("Listening on http://{}", listening);
  server.run();
}
