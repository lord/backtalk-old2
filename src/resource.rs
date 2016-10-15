use serde::{Serialize, Deserialize};
use serde_json;
use futures::{finished, Future, BoxFuture};
use ::params::Params;
use http;

pub type Reply<T: Resource> = BoxFuture<T::Object, T::Error>;
pub type ListReply<T: Resource> = BoxFuture<Vec<T::Object>, T::Error>;

pub trait Resource: Sized + 'static + Send {
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
}

impl <T: Resource + Send> ResourceWrapper for T {
  fn handle(&self, id_str: Option<&str>) -> BoxFuture<http::Message<http::Response>, http::Error> {
      // let path = Url::parse(uri).expect("MEOW3");

      self.find(&::params::Params::new()).then(|res| {
        let resp_string = match res {
          Ok(i) => serde_json::to_string(&i).unwrap(),
          Err(i) => serde_json::to_string(&i).unwrap(),
        };

        // Create the HTTP response with the body
        Ok(http::Message::new(http::Response::ok()).with_body(resp_string.into_bytes()))
      }).boxed()
  }
}

pub trait ResourceWrapper: Send + 'static {
    fn handle(&self, Option<&str>) -> BoxFuture<http::Message<http::Response>, http::Error>;
}
