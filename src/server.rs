use http;
use hyper;
use std::collections::HashMap;
use ::resource::ResourceWrapper;
use tokio_service::Service;
use futures::{BoxFuture, Async};
use ::ErrorHandler;
use ::error;
use ::Value;
use ::Request;
use ::RequestType;
use serde_json;

// #[derive(Clone)]
pub struct Server {
  resources: HashMap<String, Box<ResourceWrapper>>,
  error_handler: Box<ErrorHandler>,
}

impl Server {
  pub fn new() -> Server {
    Server {
      resources: HashMap::new(),
      error_handler: Box::new(error::DefaultErrorHandler),
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
    type Request = http::Message<http::Request>;
    type Response = http::Message<http::Response>;
    type Error = http::Error;
    type Future = BoxFuture<Self::Response, http::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
      let (head, body_buf) = req.deconstruct();
      let body_string = match String::from_utf8(body_buf) {
        Ok(val) => val,
        Err(_) => return self.error_handler.handle("Invalid utf8 in request body."),
      };
      let mut uri = if let &hyper::uri::RequestUri::AbsolutePath{ref path, ref query} = head.uri() {
        path.split('/').skip(1)
      } else {
        return self.error_handler.handle("Invalid request path.");
      };

      let resource_name = uri.next().unwrap(); // difficult to break this since split always returns at least ""
      let resource_id = if let Some(resource_id) = uri.next() {
        if resource_id.is_empty() {
          None
        } else {
          match serde_json::from_str::<Value>(&resource_id) {
            Ok(val) => Some(val),
            Err(_) => return self.error_handler.handle("Invalid JSON in id field"),
          }
        }
      } else {
        None
      };

      let body_val = if body_string.is_empty() {
        None
      } else {
        Some(match serde_json::from_str::<Value>(&body_string) {
          Ok(val) => val,
          Err(_) => return self.error_handler.handle("Invalid JSON in request body"),
        })
      };

      let resource = match self.resources.get(resource_name) {
          Some(resource) => resource,
          None => return self.error_handler.handle("No resource with that name known."),
      };

      let params = ::Params::new(); // TODO ACTUALLY PARSE

      let request_type = match head.method() {
        // TODO should we handle HEAD requests?
        &hyper::method::Method::Get => {
          if resource_id != None {
            RequestType::Get
          } else {
            RequestType::Find
          }
        }
        &hyper::method::Method::Post => RequestType::Create,
        &hyper::method::Method::Put => RequestType::Update,
        &hyper::method::Method::Patch => RequestType::Patch,
        &hyper::method::Method::Delete => RequestType::Remove,
        _ => return self.error_handler.handle("We don't respond to that HTTP method, sorry."),
      };

      let req = Request{
        request_type: request_type,
        params: params,
        id: resource_id,
        object: body_val,
      };

      if !req.validate() {
        return self.error_handler.handle("Invalid request, missing either a body or id or something.")
      }

      resource.handle(req)
    }

    fn poll_ready(&self) -> Async<()> {
        Async::Ready(())
    }
}

