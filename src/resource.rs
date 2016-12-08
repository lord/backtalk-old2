use serde::{Serialize, Deserialize};
use futures::{finished, Future, BoxFuture};
use ::params::Params;
use ::{Request, Value, Error};

pub type Reply<T: Resource> = BoxFuture<T::Object, Error>;
pub type ListReply<T: Resource> = BoxFuture<Vec<T::Object>, Error>;

pub trait Resource: Sized + 'static + Send {
  type Object: Serialize + Deserialize + 'static + Send;
  type Id: Serialize + Deserialize + 'static + Send;

  fn find(&self, &Params) -> ListReply<Self>;
  fn get(&self, &Self::Id, &Params) -> Reply<Self>;
  fn create(&self, &Self::Object, &Params) -> Reply<Self>;
  fn update(&self, &Self::Id, &Self::Object, &Params) -> Reply<Self>;
  fn patch(&self, &Self::Id, &Self::Object, &Params) -> Reply<Self>;
  fn remove(&self, &Self::Id, &Params) -> Reply<Self>;

  fn resp(&self, obj: Vec<Self::Object>) -> ListReply<Self> {
    finished::<Vec<Self::Object>, Error>(obj).boxed()
  }
}
