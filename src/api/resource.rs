use serde::{Serialize, Deserialize};
use futures::{finished, Future, BoxFuture};
use ::api::Params;
use ::api::{Request, Error, Value, RequestType};
use serde_json::value::{from_value, to_value};

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

  fn handle(&self, r: Request) -> BoxFuture<Value, Error> {
    // TODO return error unless r.validate() == true
    let i = r.id.and_then(|v| from_value::<Self::Id>(v).ok()); // TODO HANDLE DESERIALIZATION FAILURE
    let o = r.object.and_then(|v| from_value::<Self::Object>(v).ok()); // TODO HANDLE DESERIALIZATION FAILURE
    let p = r.params;
    let res = match r.request_type {
      RequestType::Get => self.get(&i.unwrap(), &p),
      RequestType::Create => self.create(&o.unwrap(), &p),
      RequestType::Update => self.update(&i.unwrap(), &o.unwrap(), &p),
      RequestType::Patch => self.patch(&i.unwrap(), &o.unwrap(), &p),
      RequestType::Remove => self.remove(&i.unwrap(), &p),
      RequestType::Find => {
        return self.find(&p).then(move |res| {
          match res {
            Ok(val) => Ok(to_value(val)),
            Err(e) => Err(e),
          }
        }).boxed()
      },
    };
    res.then(move |res| {
      match res {
        Ok(val) => Ok(to_value(val)),
        Err(e) => Err(e),
      }
    }).boxed()
  }
}
