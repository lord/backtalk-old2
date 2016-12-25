extern crate backtalk;
extern crate futures;
extern crate hyper;
extern crate serde_json;

use std::collections::HashMap;
use backtalk::api::{Resource, Params, ErrorKind, Error, Request, Value, wrap_api, serialize};
use futures::{Future, finished, failed, BoxFuture};
use serde_json::value::to_value;

struct MyResource;

impl Resource for MyResource {
  fn find(&self, _: &Params) -> BoxFuture<Value, Error> {
    let mut map = HashMap::new();
    map.insert("test".to_string(), "blah".to_string());
    let v = vec![map];
    finished(to_value(v)).boxed()
  }

  fn get(&self, _: Value, _: &Params) -> BoxFuture<Value, Error> {
    let mut map = HashMap::new();
    map.insert("test".to_string(), "blah".to_string());
    finished(to_value(map)).boxed()
  }

  fn create(&self, obj: Value, p: &Params) -> BoxFuture<Value, Error> {
    serialize(obj, |map: HashMap<String, String>| {
      let mystr = format!("{:?}", map);
      finished(mystr).boxed()
    })
  }
  fn update(&self, _: Value, _: Value, p: &Params) -> BoxFuture<Value, Error> {
    self.get(Value::U64(1), p)
  }
  fn patch(&self, _: Value, _: Value, p: &Params) -> BoxFuture<Value, Error> {
    self.get(Value::U64(1), p)
  }
  fn remove(&self, _: Value, p: &Params) -> BoxFuture<Value, Error> {
    self.get(Value::U64(1), p)
  }
}

fn example_guard(req: Request) -> BoxFuture<Request, Error> {
  if req.id() == Some(&Value::U64(3)) {
    failed(Error{msg: "access denied".to_string(), kind: ErrorKind::RemoveThis}).boxed()
  } else {
    finished(req).boxed()
  }
}

fn main() {
  backtalk::server("127.0.0.1:1337", || {
    move |http_req| {
      wrap_api(http_req, |req| {
        match req.resource.as_ref() {
          "myresource" => {
            finished(req)
              .and_then(|req| example_guard(req))
              .and_then(|req| MyResource{}.handle(req))
              .boxed()
          },
          _ => {
            failed(Error{msg: "couldn't find that resource".to_string(), kind: ErrorKind::RemoveThis})
              .boxed()
          },
        }
      })
    }
  });
}
