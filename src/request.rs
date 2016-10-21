use ::{Params, Value};

#[derive(Debug)]
pub enum RequestType{
  Find,
  Get,
  Create,
  Update,
  Patch,
  Remove,
}

use RequestType::*;

#[derive(Debug)]
pub struct Request {
  pub request_type: RequestType,
  pub params: Params,
  pub object: Option<Value>,
  pub id: Option<Value>,
}

impl Request {
  pub fn validate(&self) -> bool {
    // check object field
    match self.request_type {
      Create | Update | Patch => {
        if self.object == None {
          return false;
        }
      },
      _ => {
        if self.object != None {
          return false;
        }
      }
    }

    // check id field
    match self.request_type {
      Get | Update | Patch | Remove => {
        if self.id == None {
          return false;
        }
      },
      _ => {
        if self.id != None {
          return false;
        }
      }
    }

    true
  }
}
