use hyper;
use hyper::server as http;

pub struct Error {
  pub kind: ErrorKind,
  pub msg: String,
}

pub enum ErrorKind {
  Forbidden,
  NotFound,
  RemoveThis,
}
