use std::collections::HashMap;

pub struct Params {
  data: HashMap<String,String>
}

impl Params {
  pub fn new() -> Params {
    return Params {
      data: HashMap::<String,String>::new()
    }
  }

  pub fn get(&self, key: &str) -> Option<&String> {
    return self.data.get(key)
  }
}
