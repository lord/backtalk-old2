use http;
use serde::{Serialize, Deserialize};
use serde_json;
use tokio_service::Service;
use futures::{finished, Future, BoxFuture, Async};

// #[derive(Clone)]
pub struct ResourceServer<T: Resource> {
  resource: T,
}

pub type Reply<T: Resource> = BoxFuture<T::Object, T::Error>;
pub type ListReply<T: Resource> = BoxFuture<Vec<T::Object>, T::Error>;

pub trait Resource: Sized {
  type Object: Serialize + Deserialize + 'static + Send;
  type Error: Serialize + Deserialize + 'static + Send;

  fn list(&self) -> ListReply<Self>;
  fn obj(&self) -> Reply<Self>;

  fn resp(&self, obj: Vec<Self::Object>) -> ListReply<Self> {
    finished::<Vec<Self::Object>, Self::Error>(obj).boxed()
  }

  fn serve(self) -> ResourceServer<Self> {
    ResourceServer{
      resource: self,
    }
  }
}

impl <T: Resource> Service for ResourceServer<T> {
    type Request = http::Message<http::Request>;
    type Response = http::Message<http::Response>;
    type Error = http::Error;
    type Future = BoxFuture<Self::Response, http::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
      // let (head, body_buf) = req.deconstruct();
      // let body_string = String::from_utf8(body_buf).unwrap();
      // let uri = if let AbsolutePath(path) = head.uri() {
      //   path
      // } else {
      //   panic!()
      // };

      // let body = serde_json::from_str<T::Object>(&body_string);

      self.resource.list().then(|res| {
        let resp_string = match res {
          Ok(i) => serde_json::to_string(&i).unwrap(),
          Err(i) => serde_json::to_string(&i).unwrap(),
        };

        // Create the HTTP response with the body
        Ok(http::Message::new(http::Response::ok()).with_body(resp_string.into_bytes()))
      }).boxed()
    }

    fn poll_ready(&self) -> Async<()> {
        Async::Ready(())
    }
}
