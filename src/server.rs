use hyper::server as http;
use hyper;
use std::collections::HashMap;
use ::resource::ResourceWrapper;
use tokio_service::Service;
use futures::{BoxFuture, Async, Future, finished};
use ::Error;
use ::Value;
use ::Request;
use ::RequestType;
use serde_json;

pub trait ErrorHandler: 'static + Send {
  fn handle_http(&self, Error) -> BoxFuture<http::Response, hyper::Error>;
}

struct DefaultErrorHandler;

impl ErrorHandler for DefaultErrorHandler {
  fn handle_http(&self, err: Error) -> BoxFuture<http::Response, hyper::Error> {
    let resp = http::Response::new().body(err.msg.into_bytes());
    finished(resp).boxed()
  }
}

// #[derive(Clone)]
pub struct Server {
  resources: HashMap<String, Box<ResourceWrapper>>,
  error_handler: Box<ErrorHandler>,
}

impl Server {
  pub fn new() -> Server {
    Server {
      resources: HashMap::new(),
      error_handler: Box::new(DefaultErrorHandler),
    }
  }

  pub fn resource<T: ResourceWrapper>(&mut self, name: &str, resource: T) {
    self.resources.insert(name.to_string(), Box::new(resource));
  }

  pub fn error<T: ErrorHandler>(&mut self, handler: T) {
    self.error_handler = Box::new(handler);
  }
}

impl Service for Server {
    type Request = http::Request;
    type Response = http::Response;
    type Error = hyper::Error;
    type Future = BoxFuture<Self::Response, hyper::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
      let path = req.path();
      // let body_string = match String::from_utf8(req.body().collect()) {
      //   Ok(val) => val,
      //   Err(_) => return self.error_handler.handle_http(Error {
      //   msg: "Invalid utf8 in request body.".to_string(),
      //   kind: ::error::ErrorKind::RemoveThis
      // }),
      // };
      let mut uri = if let Some(path) = req.path() {
        path.split('/').skip(1)
      } else {
        return self.error_handler.handle_http(Error {
          msg: "Invalid request path.".to_string(),
          kind: ::error::ErrorKind::RemoveThis
        });
      };

      let resource_name = uri.next().unwrap(); // difficult to break this since split always returns at least ""
      let resource_id = if let Some(resource_id) = uri.next() {
        if resource_id.is_empty() {
          None
        } else {
          match serde_json::from_str::<Value>(&resource_id) {
            Ok(val) => Some(val),
            Err(_) => return self.error_handler.handle_http(Error {
              msg: "Invalid JSON in id field".to_string(),
              kind: ::error::ErrorKind::RemoveThis
            }),
          }
        }
      } else {
        None
      };

      let body_val = None; // if body_string.is_empty() {
      //   None
      // } else {
      //   Some(match serde_json::from_str::<Value>(&body_string) {
      //     Ok(val) => val,
      //     Err(_) => return self.error_handler.handle_http(Error {
      //   msg: "Invalid JSON in request body".to_string(),
      //   kind: ::error::ErrorKind::RemoveThis
      // }),
      //   })
      // };

      let resource = match self.resources.get(resource_name) {
          Some(resource) => resource,
          None => return self.error_handler.handle_http(Error {
            msg: "No resource with that name known.".to_string(),
            kind: ::error::ErrorKind::RemoveThis
          }),
      };

      let params = ::Params::new(); // TODO ACTUALLY PARSE

      let request_type = match req.method() {
        // TODO should we handle HEAD requests?
        &hyper::Method::Get => {
          if resource_id != None {
            RequestType::Get
          } else {
            RequestType::Find
          }
        }
        &hyper::Method::Post => RequestType::Create,
        &hyper::Method::Put => RequestType::Update,
        &hyper::Method::Patch => RequestType::Patch,
        &hyper::Method::Delete => RequestType::Remove,
        _ => return self.error_handler.handle_http(Error {
          msg: "We don't respond to that HTTP method, sorry.".to_string(),
          kind: ::error::ErrorKind::RemoveThis
        }),
      };

      let req = Request{
        request_type: request_type,
        params: params,
        id: resource_id,
        object: body_val,
      };

      if !req.validate() {
        return self.error_handler.handle_http(Error {
          msg: "Invalid request, missing either a body or id or something.".to_string(),
          kind: ::error::ErrorKind::RemoveThis
        })
      }

      resource.handle(req)
    }
}

