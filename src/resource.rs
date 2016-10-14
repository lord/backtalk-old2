use http;
use serde::Serialize;
use serde_json;
use tokio_service::Service;
use futures::{Future, BoxFuture, Async};

// #[derive(Clone)]
pub struct ResourceServer<T: Resource> {
  resource: T,
}

pub trait Resource: Sized {
  type Object: Serialize + 'static;
  type Error: Serialize + 'static;

  fn list(&self) -> BoxFuture<Vec<Self::Object>, Self::Error>;
  fn obj(&self) -> BoxFuture<Self::Object, Self::Error>;

  fn serve(self) -> ResourceServer<Self> {
    ResourceServer{
      resource: self,
    }
  }
}

impl <T: Resource> Service for ResourceServer<T> {
    type Request = http::Message<http::Request>;
    type Response = http::Message<http::Response>;
    type Error = http::Error;
    type Future = BoxFuture<Self::Response, http::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        println!("REQUEST: {:?}", req);

        self.resource.list().then(|res| {
          let resp_string = match res {
            // TODO GET RID OF UNWRAPS
            Ok(i) => serde_json::to_string(&i).unwrap(),
            Err(i) => serde_json::to_string(&i).unwrap(),
          };

          // Create the HTTP response with the body
          Ok(http::Message::new(http::Response::ok()).with_body(resp_string.into_bytes()))
        }).boxed()
    }

    fn poll_ready(&self) -> Async<()> {
        Async::Ready(())
    }
}
