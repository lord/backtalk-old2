use futures::{finished, Future, BoxFuture};
use http;

pub trait ErrorHandler: 'static + Send {
  fn handle(&self) -> BoxFuture<http::Message<http::Response>, http::Error>;
}

pub struct DefaultErrorHandler;

impl ErrorHandler for DefaultErrorHandler {
  fn handle(&self) -> BoxFuture<http::Message<http::Response>, http::Error> {
    let resp = http::Message::new(http::Response::ok()).with_body("error!".to_string().into_bytes());
    finished(resp).boxed()
  }
}
