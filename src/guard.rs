use futures::BoxFuture;
use ::{Request, Error};

pub trait Guard {
  fn handle(&self, Request) -> BoxFuture<Request, Error>;
}
