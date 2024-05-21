// key value structure
#[derive(Debug)]
struct KV {
    key: String,
    value: String
}

#[derive(Debug)]
pub struct Store {
    data: Vec<KV>
}
impl Store {
    pub fn new() -> Self {
        Store { 
            data: Vec::new() 
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.data.push(KV { key, value });
    }

    pub fn get(&self, key: &str) -> Result<&String, &str> {
        for pair in &self.data {
            if pair.key == key {
                return Ok(&pair.value);
            }
        }
        Err("Key not found")
    }
}