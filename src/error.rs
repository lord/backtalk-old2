use futures::{finished, Future, BoxFuture};
use hyper;
use hyper::server as http;

pub trait ErrorHandler: 'static + Send {
  fn handle(&self, Error) -> BoxFuture<http::Response, hyper::Error>;
}

pub struct DefaultErrorHandler;

impl ErrorHandler for DefaultErrorHandler {
  fn handle(&self, err: Error) -> BoxFuture<http::Response, hyper::Error> {
    let resp = http::Response::new().body(err.msg.into_bytes());
    finished(resp).boxed()
  }
}

pub struct Error {
  pub kind: ErrorKind,
  pub msg: String,
}

pub enum ErrorKind {
  Forbidden,
  NotFound,
  RemoveThis,
}
