// key value structure
#[derive(Debug)]
pub struct KV {
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub struct Store {
    data: Vec<KV>,
}
impl Store {
    pub fn new() -> Self {
        Store { data: Vec::new() }
    }

    pub fn insert(&mut self, key: String, value: String) -> Result<(), &str> {
        let pair = self.get(&key);
        match pair {
            Ok(_) => Err("Key already exists"),
            Err(_) => Ok(self.data.push(KV { key, value })),
        }
    }

    pub fn get(&mut self, key: &str) -> Result<&mut KV, &str> {
        for pair in &mut self.data {
            if pair.key == key {
                return Ok(pair);
            }
        }
        Err("Key not found")
    }

    pub fn update(&mut self, key: &str, value: &str) -> Result<(), &str> {
        let entity = &mut self.get(key);
        match entity {
            Ok(pair) => {
                pair.value = value.to_string();
                return Ok(());
            }
            Err(e) => Err(e),
        }
    }

    pub fn delete(&mut self, key: &str) {
        let index = &mut self.data.iter().position(|x| *x.key == *key).unwrap();
        let _ = &mut self.data.remove(*index);
    }
}
