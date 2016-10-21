use serde::{Serialize, Deserialize};
use serde_json;
use futures::{finished, Future, BoxFuture};
use ::params::Params;
use ::Request;
use serde_json::value::from_value;
use http;

pub type Reply<T: Resource> = BoxFuture<T::Object, T::Error>;
pub type ListReply<T: Resource> = BoxFuture<Vec<T::Object>, T::Error>;

fn serialize_result<A: Serialize, B: Serialize>(r: Result<A, B>) -> Result<String,String> {
  match r {
    Ok(i) => Ok(serde_json::to_string(&i).unwrap()),
    Err(i) => Err(serde_json::to_string(&i).unwrap()),
  }
}

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
  fn handle(&self, r: Request) -> BoxFuture<http::Message<http::Response>, http::Error> {
    let prom = match r {
      Request::Find{params} => {
        self.find(&params).then(serialize_result).boxed()
      }
      Request::Get{params, id} => {
        let id_struct = from_value::<T::Id>(id).unwrap();
        self.get(&id_struct, &params).then(serialize_result).boxed()
      }
      Request::Create{params, object} => {
        let obj_struct = from_value::<T::Object>(object).unwrap();
        self.create(&obj_struct, &params).then(serialize_result).boxed()
      }
      Request::Update{params, id, object} => {
        let id_struct = from_value::<T::Id>(id).unwrap();
        let obj_struct = from_value::<T::Object>(object).unwrap();
        self.update(&id_struct, &obj_struct, &params).then(serialize_result).boxed()
      }
      Request::Patch{params, id, object} => {
        let id_struct = from_value::<T::Id>(id).unwrap();
        let obj_struct = from_value::<T::Object>(object).unwrap();
        self.patch(&id_struct, &obj_struct, &params).then(serialize_result).boxed()
      }
      Request::Remove{params, id} => {
        self.remove(&from_value::<T::Id>(id).unwrap(), &params).then(serialize_result).boxed()
      }
    };
    prom.then(|res| {
      match res {
        Ok(resp_string) => Ok(http::Message::new(http::Response::ok()).with_body(resp_string.into_bytes())),
        Err(resp_string) => Ok(http::Message::new(http::Response::ok()).with_body(resp_string.into_bytes())),
      }
    }).boxed()
  }
}

pub trait ResourceWrapper: Send + 'static {
    fn handle(&self, Request) -> BoxFuture<http::Message<http::Response>, http::Error>;
}
