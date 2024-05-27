use super::STORAGE_MUTEX;
use serde;

/// Represents a key-value pair.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct KV {
    pub key: String,
    pub value: String,
}

/// Represents the in-memory key-value store.
#[derive(Debug)]
pub struct Store {
    pub data: Vec<KV>,
    pub storage: Storage,
}

impl Store {
    /// Creates a new, empty `Store`.
    ///
    /// # Returns
    /// A new instance of `Store`.
    pub fn new() -> Self {
        Store {
            data: Vec::new(),
            storage: Storage::new(None),
        }
    }

    /// Inserts a key-value pair into the store.
    /// and save the new data into file
    ///
    /// # Arguments
    /// * `key` - The key to insert.
    /// * `value` - The value to associate with the key.
    ///
    /// # Returns
    /// * `Ok(())` if the insertion is successful.
    /// * `Err(&str)` if the key already exists.
    pub fn insert(&mut self, key: &str, value: &str) -> Result<(), String> {
        let pair = self.get(&key);
        match pair {
            Ok(_) => Err("Key already exists".to_string()),
            Err(_) => {
                self.data.push(KV {
                    key: key.to_string(),
                    value: value.to_string(),
                });
                if let Err(e) = self.persist_data() {
                    return Err(e.to_string());
                }
                Ok(())
            }
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
    /// and save the new data into file
    ///
    /// # Arguments
    /// * `key` - The key to update.
    /// * `value` - The new value to associate with the key.
    ///
    /// # Returns
    /// * `Ok(())` if the update is successful.
    /// * `Err(String)` if the key is not found.
    pub fn update(&mut self, key: &str, value: &str) -> Result<(), String> {
        let entity = self.get(key);
        match entity {
            Ok(pair) => {
                pair.value = value.to_string();
                self.persist_data().unwrap();
                Ok(())
            }
            Err(e) => Err(e.to_string()),
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
        self.persist_data().unwrap();
    }

    /// Persists the current data to the storage by serializing it and writing it to a file.
    ///
    /// This method clones the current data in the storage, serializes it into a binary format,
    /// and writes it to the associated file. If the file operation is successful, it returns `Ok(())`.
    /// If there is an error during the file operation, it returns an error message.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the data is successfully persisted to the file.
    /// * `Err(&str)` - If there is an error during the file operation, with an error message.
    ///
    /// # Errors
    ///
    /// This method returns an error in the following situations:
    ///
    /// * If there is an issue with the file operation (e.g., unable to write to the file).
    fn persist_data(&mut self) -> Result<(), &str> {
        let d = self.data.clone(); // Clone the current data to avoid borrowing issues.
        let mut storage = STORAGE_MUTEX.lock().unwrap();
        let result = storage.save_file(d); // Attempt to save the cloned data to the file.
        match result {
            Ok(_) => Ok(()), // Return Ok if the file operation is successful.
            Err(_) => Err("Failed to persist data."), // Return an error message if the file operation fails.
        }
    }
}
