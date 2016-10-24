use futures::BoxFuture;
use ::{Request, ErrorHandler};

pub trait Guard {
  fn handle(&self, Request) -> BoxFuture<Request, ErrorHandler>;
}
