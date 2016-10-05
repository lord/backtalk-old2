extern crate tokio_service;
extern crate tokio_hyper as http;
extern crate futures;
mod backtalk;

use tokio_service::Service;
use futures::{Future, BoxFuture, finished, Async};

#[derive(Clone)]
struct ServiceWrapper;

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


#[cfg(test)]
mod tests {
    use super::ServiceWrapper;
    use http;
    use std::time::Duration;
    use std::thread;
    #[test]
    fn it_works() {
        http::Server::new()
            .serve(|| ServiceWrapper)
            .unwrap();

        thread::sleep(Duration::from_secs(1_000_000));
    }
}
