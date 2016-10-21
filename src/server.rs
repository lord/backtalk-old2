use http;
use hyper;
use std::collections::HashMap;
use ::resource::ResourceWrapper;
use tokio_service::Service;
use futures::{BoxFuture, Async};
use ::params::Params;
use ::ErrorHandler;
use ::error;
use ::Value;

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
        Err(_) => return self.error_handler.handle(),
      };
      let mut uri = if let &hyper::uri::RequestUri::AbsolutePath{ref path, ref query} = head.uri() {
        path.split('/').skip(1)
      } else {
        return self.error_handler.handle();
      };

      let resource_name = uri.next().unwrap(); // difficult to break this since split always returns at least ""
      let resource_id = uri.next();
      let body_val = if (body_string.is_empty()) {
        None
      } else {
        Some(match serde_json::from_str::<Value>(&body_string) {
          Ok(val) => val,
          Err(_) => return self.error_handler.handle(),
        })
      };

      match self.resources.get(resource_name) {
          Some(resource) => resource.handle(&Params::new(), resource_id, Some(&body_string)),
          None => return self.error_handler.handle(),
      }

      // parse query string
      // check first part of URL and route to approprate resource
      // check second part of URL and set ID if approprate
      // if other parts of URL, BAD REQUEST
      // check HTTP method and route to approprate method based on this and ID
        // parse body for create/update/patch if present into Resource::Object
        // call approprate method on the

      // let body = serde_json::from_str<T::Object>(&body_string);
    }

    fn poll_ready(&self) -> Async<()> {
        Async::Ready(())
    }
}

