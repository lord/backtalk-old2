use http;
use serde::{Serialize, Deserialize};
use serde_json;
use tokio_service::Service;
use futures::{finished, Future, BoxFuture, Async};
use ::params::Params;
use hyper;

// #[derive(Clone)]
pub struct ResourceServer<T: Resource> {
  resource: T,
}

pub type Reply<T: Resource> = BoxFuture<T::Object, T::Error>;
pub type ListReply<T: Resource> = BoxFuture<Vec<T::Object>, T::Error>;

pub trait Resource: Sized {
  type Object: Serialize + Deserialize + 'static + Send;
  type Error: Serialize + Deserialize + 'static + Send;
  type Id: Serialize + Deserialize + 'static + Send;

  fn find(&self, &Params) -> ListReply<Self>;
  fn get(&self, &Self::Id, &Params) -> Reply<Self>;
  fn create(&self, &Self::Object, &Params) -> Reply<Self>;
  fn update(&self, &Self::Id, &Self::Object, &Params) -> Reply<Self>;
  fn patch(&self, &Self::Id, &Self::Object, &Params) -> Reply<Self>;
  fn remove(&self, &Self::Id, &Params) -> Reply<Self>;

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
      let (head, body_buf) = req.deconstruct();
      let body_string = String::from_utf8(body_buf).expect("meow2");
      let mut uri = if let &hyper::uri::RequestUri::AbsolutePath{ref path, ref query} = head.uri() {
        path.split('/').skip(1)
      } else {
        // TODO DEAL WITH THIS
        panic!("wasn't absolute path")
      };

      let resource_name = uri.next().unwrap(); // difficult to break this since split always returns at least ""
      let resource_id = uri.next();

      println!("name: {:?}, id: {:?}", resource_name, resource_id);

      // let path = Url::parse(uri).expect("MEOW3");

      // parse query string
      // check first part of URL and route to approprate resource
      // check second part of URL and set ID if approprate
      // if other parts of URL, BAD REQUEST
      // check HTTP method and route to approprate method based on this and ID
        // parse body for create/update/patch if present into Resource::Object
        // call approprate method on the

      // let body = serde_json::from_str<T::Object>(&body_string);

      self.resource.find(&::params::new()).then(|res| {
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
