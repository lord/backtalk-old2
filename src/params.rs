use std::collections::HashMap;

pub struct Params {
  data: HashMap<String,String>
}

pub fn new() -> Params {
  return Params {
    data: HashMap::<String,String>::new()
  }
}

impl Params {
  fn get(&self, key: &str) -> Option<&String> {
    return self.data.get(key)
  }
}
