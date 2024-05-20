// key value structure
struct KV {
    key: String,
    value: String
}

struct Store {
    data: Vec<KV>
}
impl Store {
    fn new() -> Self {
        Store { 
            data: Vec::new() 
        }
    }

    fn insert(&mut self, key: String, value: String) {
        self.data.push(KV { key, value });
    }

    fn get(&self, key: &str) -> Option<&String> {
        for pair in &self.data {
            if pair.key == key {
                return Some(&pair.value);
            }
        }
        None
    }
}

fn main() {
    let mut store = Store::new();

    // Insert key-value pairs
    store.insert("name".to_string(), "SafinaDB".to_string());
    store.insert("version".to_string(), "1.0".to_string());

    // Retrieve values
    if let Some(value) = store.get("name") {
        println!("Key: name, Value: {}", value);
    } else {
        println!("Key not found");
    }
}
