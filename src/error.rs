use futures::{finished, Future, BoxFuture};
use http;

pub trait ErrorHandler: 'static + Send {
  fn handle(&self, &str) -> BoxFuture<http::Message<http::Response>, http::Error>;
}

pub struct DefaultErrorHandler;

impl ErrorHandler for DefaultErrorHandler {
  fn handle(&self, s: &str) -> BoxFuture<http::Message<http::Response>, http::Error> {
    let resp = http::Message::new(http::Response::ok()).with_body(s.to_string().into_bytes());
    finished(resp).boxed()
  }
}
