extern crate backtalk;
extern crate futures;
extern crate hyper;

use hyper::server::Server as HTTPServer;
use std::collections::HashMap;
use backtalk::api::{Resource, Reply, ListReply, Params, Router, ErrorKind, Error, Request, Value};
use backtalk::{Server, wrap_api};
use futures::{Future, finished, failed, BoxFuture};

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

fn example_guard(req: Request) -> BoxFuture<Request, Error> {
  if req.id == Some(Value::U64(3)) {
    failed(Error{msg: "access denied".to_string(), kind: ErrorKind::RemoveThis}).boxed()
  } else {
    finished(req).boxed()
  }
}

fn main() {
  let server = HTTPServer::http(&"127.0.0.1:1337".parse().unwrap()).unwrap();
  let (listening, server) = server.standalone(|| {
    Ok(Server::new(move |http_req| {
      let mut router = Router::new();
      router.add("myresource", |req| {
        finished(req)
          .and_then(|req| example_guard(req))
          .and_then(|req| MyResource{}.handle(req))
          .boxed()
      });
      wrap_api(http_req, move |req| {
        router.handle(req)
      })
    }))
  }).unwrap();
  println!("Listening on http://{}", listening);
  server.run();
}
