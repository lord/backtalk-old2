use http;
use hyper;
use std::collections::HashMap;
use ::resource::ResourceWrapper;
use tokio_service::Service;
use futures::{BoxFuture, Async};

// #[derive(Clone)]
pub struct Server {
  resources: HashMap<String, Box<ResourceWrapper>>,
}

impl Server {
  pub fn new() -> Server {
    Server {
      resources: HashMap::new()
    }
  }

  pub fn resource<T: ResourceWrapper>(&mut self, name: &str, resource: T) {
    self.resources.insert(name.to_string(), Box::new(resource));
  }
}

impl Service for Server {
    type Request = http::Message<http::Request>;
    type Response = http::Message<http::Response>;
    type Error = http::Error;
    type Future = BoxFuture<Self::Response, http::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
      let (head, body_buf) = req.deconstruct();
      let body_string = String::from_utf8(body_buf).expect("meow2");
      let mut uri = if let &hyper::uri::RequestUri::AbsolutePath{ref path, ref query} = head.uri() {
        path.split('/').skip(1)
      } else {
        // TODO DEAL WITH THIS
        panic!("wasn't absolute path")
      };

      let resource_name = uri.next().unwrap(); // difficult to break this since split always returns at least ""
      let resource_id = uri.next();

      match self.resources.get(resource_name) {
          Some(resource) => resource.handle(resource_id, Some(&body_string)),
          None => panic!("no resource with that name"),
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

