use ::{Params, Value};

pub enum Request {
  Find {
    params: Params,
  },
  Get {
    params: Params,
    id: Value,
  },
  Create {
    params: Params,
    object: Value,
  },
  Update {
    params: Params,
    id: Value,
    object: Value,
  },
  Patch {
    params: Params,
    id: Value,
    object: Value,
  },
  Remove {
    params: Params,
    id: Value,
  },
}

impl Request {
  pub fn params(&mut self) -> &mut Params {
    match self {
      &mut Request::Find{ref mut params, ..} => params,
      &mut Request::Get{ref mut params, ..} => params,
      &mut Request::Create{ref mut params, ..} => params,
      &mut Request::Update{ref mut params, ..} => params,
      &mut Request::Patch{ref mut params, ..} => params,
      &mut Request::Remove{ref mut params, ..} => params,
    }
  }

  pub fn id(&mut self) -> Option<&mut Value> {
    match self {
      &mut Request::Get{ref mut id, ..} => Some(id),
      &mut Request::Update{ref mut id, ..} => Some(id),
      &mut Request::Patch{ref mut id, ..} => Some(id),
      &mut Request::Remove{ref mut id, ..} => Some(id),
      _ => None,
    }
  }

  pub fn object(&mut self) -> Option<&mut Value> {
    match self {
      &mut Request::Create{ref mut object, ..} => Some(object),
      &mut Request::Update{ref mut object, ..} => Some(object),
      &mut Request::Patch{ref mut object, ..} => Some(object),
      _ => None,
    }
  }
}
