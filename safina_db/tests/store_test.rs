use once_cell::sync::Lazy;
use safina_db::{Store, STORAGE_MUTEX};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(test)]
pub static TEST_STORE: Lazy<Mutex<Store>> = Lazy::new(|| {
    // Perform any setup needed before tests run
    let mut store = Store::new();
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let timestamp = since_the_epoch.as_secs();
    let db_name = format!("db-test-{}", timestamp);
 
    println!("- Loading test database...");
    match STORAGE_MUTEX.lock().unwrap().load_file(Some(db_name.as_str())) {
        Ok(data) => store.data = data,
        Err(err) => {
            println!("Invalid: {}", err);
        }
    }
    // Initialize store or perform setup
    Mutex::new(store)
});

#[cfg(test)]
mod tests {
    use super::TEST_STORE;
    use safina_db::kv_store::KV;

    #[test]
    fn test_insert_new_key() {
        let mut store = TEST_STORE.lock().unwrap();
        let test_data = KV {
            key: String::from("key1"),
            value: String::from("value1")
        };
        let result = store.insert(&test_data.key, &test_data.value);
        assert!(result.is_ok());
        assert_eq!(store.get("key1").unwrap().value, "value1");
    }

    #[test]
    fn test_insert_empty_key() {
        let test_data = KV {
            key: String::from(""),
            value: String::from("value2")
        };
        let mut store = TEST_STORE.lock().unwrap();
        let result = store.insert(&test_data.key, &test_data.value);
        assert!(result.is_ok());
        assert_eq!(store.get("").unwrap().value, test_data.value);
    }

    #[test]
    fn test_insert_existing_key() {
        let test_data = KV {
            key: String::from("key3"),
            value: String::from("value3")
        };
        let mut store = TEST_STORE.lock().unwrap();
        store.insert(&test_data.key, &test_data.value).unwrap();
        let result = store.insert(&test_data.key, &test_data.value);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Key already exists".to_string()));
    }

    #[test]
    fn test_get_existing_key() {
        let test_data = KV {
            key: String::from("key4"),
            value: String::from("value4")
        };
        let mut store = TEST_STORE.lock().unwrap();
 
        store.insert(&test_data.key, &test_data.value).unwrap();
        let result = store.get(&test_data.key);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value, test_data.value);
    }

    #[test]
    fn test_get_non_existing_key() {
        let test_data = KV {
            key: String::from("key5"),
            value: String::from("")
        };
        let mut store = TEST_STORE.lock().unwrap();
        let result = store.get(&test_data.key);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Key not found"));
    }

    #[test]
    fn test_update_existing_key() {
        let test_data = KV {
            key: String::from("key6"),
            value: String::from("value6")
        };
        let test_data_update = KV {
            key: test_data.key.clone(),
            value: String::from("value6-updated")
        };
        let mut store = TEST_STORE.lock().unwrap();
        store.insert(&test_data.key, &test_data.value).unwrap();
        let result = store.update(&test_data_update.key, &test_data_update.value);
        assert!(result.is_ok());
        assert_eq!(store.get(&test_data.key).unwrap().value, test_data_update.value);
    }

    #[test]
    fn test_update_non_existing_key() {
        let test_data_update = KV {
            key: String::from(""),
            value: String::from("value7-updated")
        };
        let mut store = TEST_STORE.lock().unwrap();
        store.delete(&test_data_update.key);
        let result = store.update(&test_data_update.key, &test_data_update.value);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Key not found".to_string()));
    }

    #[test]
    fn test_delete_existing_key() {
        let test_data = KV {
            key: String::from("key8-should-be-deleted"),
            value: String::from("value8-should-be-deleted")
        };
        let mut store = TEST_STORE.lock().unwrap();
        store.insert(&test_data.key, &test_data.value).unwrap();
        store.delete(&test_data.key);
        let result = store.get(&test_data.key);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Key not found"));
    }

    #[test]
    fn test_delete_non_existing_key() {
        let mut store = TEST_STORE.lock().unwrap();
        store.delete("key-doesnt-exists"); // Should not panic or cause error
        let result = store.get("key-doesnt-exists");
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Key not found"));
    }

    // Additional edge cases
    #[test]
    fn test_insert_empty_value() {
        let test_data = KV {
            key: String::from("key9-with-empty-value"),
            value: String::from("")
        };
        let mut store = TEST_STORE.lock().unwrap();
        let result = store.insert(&test_data.key, &test_data.value);
        assert!(result.is_ok());
        assert_eq!(store.get(&test_data.key).unwrap().value, test_data.value);
    }

    #[test]
    fn test_insert_large_key_value() {
        let mut store = TEST_STORE.lock().unwrap();
        let large_key = "k".repeat(1000);
        let large_value = "v".repeat(1000);
        let result = store.insert(large_key.clone().as_str(), large_value.clone().as_str());
        assert!(result.is_ok());
        assert_eq!(store.get(&large_key).unwrap().value, large_value);
    }

    #[test]
    fn test_update_empty_value() {
        let test_data = KV {
            key: String::from("key10"),
            value: String::from("value10")
        };
        let mut store = TEST_STORE.lock().unwrap();
        store.insert(&test_data.key, &test_data.value).unwrap();
        let result = store.update(&test_data.key, "");
        assert!(result.is_ok());
        assert_eq!(store.get(&test_data.key).unwrap().value, "");
    }

    #[test]
    fn test_delete_empty_key() {
        let test_data = KV {
            key: String::from(""),
            value: String::from("value11-with-empty-key")
        };
        let mut store = TEST_STORE.lock().unwrap();
        store.delete(&test_data.key);
        store.insert(&test_data.key, &test_data.value).unwrap();
        store.delete(&test_data.key);
        let result = store.get(&test_data.key);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Key not found"));
    }

    #[test]
    fn test_delete_multiple_times() {
        let test_data = KV {
            key: String::from("key-12"),
            value: String::from("value12")
        };
        let mut store = TEST_STORE.lock().unwrap();
        store.insert(&test_data.key, &test_data.value).unwrap();
        store.delete(&test_data.key);
        store.delete(&test_data.key);
        let result = store.get(&test_data.key);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Key not found"));
    }
}
