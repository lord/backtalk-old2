use futures::{finished, Future, BoxFuture};
use hyper;
use hyper::server as http;

pub trait ErrorHandler: 'static + Send {
  fn handle(&self, &str) -> BoxFuture<http::Response, hyper::Error>;
}

pub struct DefaultErrorHandler;

impl ErrorHandler for DefaultErrorHandler {
  fn handle(&self, s: &str) -> BoxFuture<http::Response, hyper::Error> {
    let resp = http::Response::new().body(s.to_string().into_bytes());
    finished(resp).boxed()
  }
}
