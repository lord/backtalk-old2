use ::{Resource, Request, Value, Error, ErrorKind, RequestType};
use futures::{BoxFuture, Future, failed};
use tokio_service::Service;
use std::collections::HashMap;
use serde_json;

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
}

impl Service for Router {
  type Request = Request;
  type Response = Value;
  type Error = Error;
  type Future = BoxFuture<Self::Response, Self::Error>;

  fn call(&self, req: Request) -> BoxFuture<Self::Response, Self::Error> {
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

// impl <T: Resource + Sync> ResourceWrapper for T {
//   fn handle(&self, r: Request) -> BoxFuture<Value, Error> {
//     // TODO return error unless r.validate() == true
//     let i = r.id.and_then(|v| from_value::<T::Id>(v).ok()); // TODO HANDLE DESERIALIZATION FAILURE
//     let o = r.object.and_then(|v| from_value::<T::Object>(v).ok()); // TODO HANDLE DESERIALIZATION FAILURE
//     let p = r.params;
//     let res = match r.request_type {
//       RequestType::Get => self.get(&i.unwrap(), &p),
//       RequestType::Create => self.create(&o.unwrap(), &p),
//       RequestType::Update => self.update(&i.unwrap(), &o.unwrap(), &p),
//       RequestType::Patch => self.patch(&i.unwrap(), &o.unwrap(), &p),
//       RequestType::Remove => self.remove(&i.unwrap(), &p),
//       RequestType::Find => {
//         return self.find(&p).then(move |res| {
//           match res {
//             Ok(val) => Ok(serde_json::to_value(val)),
//             Err(e) => Err(e),
//           }
//         }).boxed()
//       },
//     };
//     res.then(move |res| {
//       match res {
//         Ok(val) => Ok(serde_json::to_value(val)),
//         Err(e) => Err(e),
//       }
//     }).boxed()
//   }
// }

// trait ResourceWrapper: Sync + 'static {
//   fn handle(&self, Request) -> BoxFuture<Value, Error>;
// }
