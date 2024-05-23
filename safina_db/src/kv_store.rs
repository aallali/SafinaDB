 
/// Represents a key-value pair.
#[derive(Debug)]
pub struct KV {
    pub key: String,
    pub value: String,
}

/// Represents the in-memory key-value store.
#[derive(Debug)]
pub struct Store {
    data: Vec<KV>,
}

impl Store {
    /// Creates a new, empty `Store`.
    ///
    /// # Returns
    /// A new instance of `Store`.
    pub fn new() -> Self {
        Store { data: Vec::new() }
    }

    /// Inserts a key-value pair into the store.
    ///
    /// # Arguments
    /// * `key` - The key to insert.
    /// * `value` - The value to associate with the key.
    ///
    /// # Returns
    /// * `Ok(())` if the insertion is successful.
    /// * `Err(&str)` if the key already exists.
    pub fn insert(&mut self, key: String, value: String) -> Result<(), &str> {
        let pair = self.get(&key);
        match pair {
            Ok(_) => Err("Key already exists"),
            Err(_) => Ok(self.data.push(KV { key, value })),
        }
    }

    /// Retrieves a mutable reference to the key-value pair associated with the given key.
    ///
    /// # Arguments
    /// * `key` - The key to search for.
    ///
    /// # Returns
    /// * `Ok(&mut KV)` if the key is found.
    /// * `Err(&str)` if the key is not found.
    pub fn get(&mut self, key: &str) -> Result<&mut KV, &str> {
        for pair in &mut self.data {
            if pair.key == key {
                return Ok(pair);
            }
        }
        Err("Key not found")
    }

    /// Updates the value associated with the given key.
    ///
    /// # Arguments
    /// * `key` - The key to update.
    /// * `value` - The new value to associate with the key.
    ///
    /// # Returns
    /// * `Ok(())` if the update is successful.
    /// * `Err(&str)` if the key is not found.
    pub fn update(&mut self, key: &str, value: &str) -> Result<(), &str> {
        let entity = &mut self.get(key);
        match entity {
            Ok(pair) => {
                pair.value = value.to_string();
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Deletes the key-value pair associated with the given key.
    ///
    /// # Arguments
    /// * `key` - The key to delete.
    pub fn delete(&mut self, key: &str) {
        if let Some(index) = self.data.iter().position(|x| x.key == key) {
            self.data.remove(index);
        }
    }
}
