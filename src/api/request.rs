use ::api::{Params, Value};

#[derive(Debug)]
pub enum RequestData{
  Find,
  Get(Value),
  Create(Value),
  Update(Value, Value),
  Patch(Value, Value),
  Remove(Value),
}

use self::RequestData::*;

#[derive(Debug)]
pub struct Request {
  pub resource: String,
  pub data: RequestData,
  pub params: Params,
}

impl Request {
  pub fn id(&self) -> Option<&Value> {
    // check object field
    match &self.data {
      &Get(ref v) => Some(v),
      &Update(ref v, _) => Some(v),
      &Patch(ref v, _) => Some(v),
      &Remove(ref v) => Some(v),
      _ => None,
    }
  }

  pub fn obj(&self) -> Option<&Value> {
    // check object field
    match &self.data {
      &Create(ref v) => Some(v),
      &Update(_, ref v) => Some(v),
      &Patch(_, ref v) => Some(v),
      _ => None,
    }
  }
}
