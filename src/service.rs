use http;
use tokio_service::Service;
use futures::{Future, BoxFuture, finished, Async};

#[derive(Clone)]
pub struct ServiceWrapper;

pub fn server() -> ServiceWrapper {
  ServiceWrapper{}
}

impl Service for ServiceWrapper {
    type Request = http::Message<http::Request>;
    type Response = http::Message<http::Response>;
    type Error = http::Error;
    type Future = BoxFuture<Self::Response, http::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        println!("REQUEST: {:?}", req);

        // Create the HTTP response with the body
        let resp = http::Message::new(http::Response::ok())
            .with_body(b"this is my message\n".to_vec());

        // Return the response as an immediate future
        finished(resp).boxed()
    }

    fn poll_ready(&self) -> Async<()> {
        Async::Ready(())
    }
}

