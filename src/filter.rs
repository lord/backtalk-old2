use futures::BoxFuture;
use ::{Request, ErrorHandler};

pub trait Filter {
  fn handle(&self, Request) -> BoxFuture<Request, ErrorHandler>;
}
