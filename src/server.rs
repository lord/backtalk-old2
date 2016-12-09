use hyper::server as http;
use hyper;
use tokio_service::Service;
use futures::BoxFuture;

pub fn server<T, U>(bind: &str, func: U)
  where U: Fn() -> T + 'static + Send,
        T: Fn(http::Request) -> BoxFuture<http::Response, hyper::Error> + 'static {

  let server = http::Server::http(&bind.parse().unwrap()).unwrap();
  let (listening, server) = server.standalone(move || {
    let srv = Server{func: func()};
    Ok(srv)
  }).unwrap();

  println!("Backtalk listening on http://{}", listening);
  server.run();
}

struct Server<T: Fn(http::Request) -> BoxFuture<http::Response, hyper::Error> + 'static> {
  func: T,
}

impl <T> Service for Server<T>
  where T: Fn(http::Request) -> BoxFuture<http::Response, hyper::Error> + 'static {
  type Request = http::Request;
  type Response = http::Response;
  type Error = hyper::Error;
  type Future = BoxFuture<Self::Response, hyper::Error>;

  fn call(&self, req: Self::Request) -> Self::Future {
    (self.func)(req)
  }
}

