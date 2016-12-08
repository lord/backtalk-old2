// use serde::{Serialize, Deserialize};
use futures::{Future, BoxFuture};
use ::api::Params;
use ::api::{Request, Error, Value, RequestData};
use serde_json::value::to_value;

pub trait Resource: Sized + 'static + Send {
  fn find(&self, &Params) -> BoxFuture<Value, Error>;
  fn get(&self, &Value, &Params) -> BoxFuture<Value, Error>;
  fn create(&self, &Value, &Params) -> BoxFuture<Value, Error>;
  fn update(&self, &Value, &Value, &Params) -> BoxFuture<Value, Error>;
  fn patch(&self, &Value, &Value, &Params) -> BoxFuture<Value, Error>;
  fn remove(&self, &Value, &Params) -> BoxFuture<Value, Error>;

  fn handle(&self, r: Request) -> BoxFuture<Value, Error> {
    // TODO return error unless r.validate() == true
    let p = r.params;
    let res = match &r.data {
      &RequestData::Get(ref id) => self.get(id, &p), // TODO HANDLE DESERIALIZATION FAILURE
      &RequestData::Create(ref obj) => self.create(obj, &p),
      &RequestData::Update(ref id, ref obj) => self.update(id, obj, &p),
      &RequestData::Patch(ref id, ref obj) => self.patch(id, obj, &p),
      &RequestData::Remove(ref id) => self.remove(id, &p),
      &RequestData::Find => self.find(&p),
    };
    res.then(move |res| {
      match res {
        Ok(val) => Ok(to_value(val)),
        Err(e) => Err(e),
      }
    }).boxed()
  }
}
