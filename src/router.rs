use ::{Request, Value, Error, ErrorKind};
use futures::{BoxFuture, Future, failed};
use std::collections::HashMap;

pub struct Router {
  resources: HashMap<String, Box<Fn(Request) -> BoxFuture<Value, Error>>>,
}

impl Router {
  pub fn new() -> Router {
    Router {
      resources: HashMap::new(),
    }
  }
  pub fn add<T>(&mut self, name: &str, resource: T)
    where T: Fn(Request) -> BoxFuture<Value, Error> + Sync + 'static {
    self.resources.insert(name.to_string(), Box::new(resource));
  }

  pub fn handle(&self, req: Request) -> BoxFuture<Value, Error> {
    let resource = match self.resources.get(&req.resource) {
      Some(res) => res,
      None => return failed(Error{
          msg: "couldn't find that resource".to_string(),
          kind: ErrorKind::RemoveThis
      }).boxed(),
    };

    resource(req)
  }
}
