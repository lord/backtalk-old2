use std::collections::HashMap;

pub struct Params {
  data: HashMap<String,String>
}

pub fn new(map: HashMap<String,String>) -> Params {
  return Params {
    data: map
  }
}

impl Params {
  fn get(&self, key: &str) -> Option<&String> {
    return self.data.get(key)
  }
}
