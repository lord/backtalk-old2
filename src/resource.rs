use http;
use tokio_service::Service;
use futures::{Future, BoxFuture, finished, Async};

// #[derive(Clone)]
pub struct ResourceServer<T: Resource> {
  resource: T,
}

pub trait Resource: Sized {
  fn text(&self) -> String;

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

        // Create the HTTP response with the body
        let resp = http::Message::new(http::Response::ok())
            .with_body(self.resource.text().into_bytes());

        // Return the response as an immediate future
        finished(resp).boxed()
    }

    fn poll_ready(&self) -> Async<()> {
        Async::Ready(())
    }
}

