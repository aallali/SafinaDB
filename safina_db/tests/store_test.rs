
#[path = "../src/kv_store.rs"] mod kv_store;

#[cfg(test)]
mod tests {
    use crate::kv_store::Store;

    #[test]
    fn test_insert_new_key() {
        let mut store = Store::new();
        let result = store.insert("key1".to_string(), "value1".to_string());
        assert!(result.is_ok());
        assert_eq!(store.get("key1").unwrap().value, "value1");
    }

    #[test]
    fn test_insert_empty_key() {
        let mut store = Store::new();
        let result = store.insert("".to_string(), "value1".to_string());
        assert!(result.is_ok());
        assert_eq!(store.get("").unwrap().value, "value1");
    }

    #[test]
    fn test_insert_existing_key() {
        let mut store = Store::new();
        store.insert("key1".to_string(), "value1".to_string()).unwrap();
        let result = store.insert("key1".to_string(), "value2".to_string());
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Key already exists"));
    }

    #[test]
    fn test_get_existing_key() {
        let mut store = Store::new();
        let expected_resp = "value1,value2,value3";
        store.insert("key1".to_string(), expected_resp.to_string()).unwrap();
        let result = store.get("key1");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value, expected_resp);
    }

    #[test]
    fn test_get_non_existing_key() {
        let mut store = Store::new();
        let result = store.get("key1");
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Key not found"));
    }

    #[test]
    fn test_update_existing_key() {
        let mut store = Store::new();
        store.insert("key1".to_string(), "value1".to_string()).unwrap();
        let result = store.update("key1", "value2");
        assert!(result.is_ok());
        assert_eq!(store.get("key1").unwrap().value, "value2");
    }

    #[test]
    fn test_update_non_existing_key() {
        let mut store = Store::new();
        let result = store.update("key1", "value1");
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Key not found"));
    }

    #[test]
    fn test_delete_existing_key() {
        let mut store = Store::new();
        store.insert("key1".to_string(), "value1".to_string()).unwrap();
        store.delete("key1");
        let result = store.get("key1");
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Key not found"));
    }

    #[test]
    fn test_delete_non_existing_key() {
        let mut store = Store::new();
        store.delete("key1"); // Should not panic or cause error
        let result = store.get("key1");
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Key not found"));
    }

    // Additional edge cases
    #[test]
    fn test_insert_empty_value() {
        let mut store = Store::new();
        let result = store.insert("key".to_string(), "".to_string());
        assert!(result.is_ok());
        assert_eq!(store.get("key").unwrap().value, "");
    }

    #[test]
    fn test_insert_large_key_value() {
        let mut store = Store::new();
        let large_key = "k".repeat(1000);
        let large_value = "v".repeat(1000);
        let result = store.insert(large_key.clone(), large_value.clone());
        assert!(result.is_ok());
        assert_eq!(store.get(&large_key).unwrap().value, large_value);
    }

    #[test]
    fn test_update_empty_value() {
        let mut store = Store::new();
        store.insert("key".to_string(), "value".to_string()).unwrap();
        let result = store.update("key", "");
        assert!(result.is_ok());
        assert_eq!(store.get("key").unwrap().value, "");
    }

    #[test]
    fn test_delete_empty_key() {
        let mut store = Store::new();
        store.insert("".to_string(), "value".to_string()).unwrap();
        store.delete("");
        let result = store.get("");
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Key not found"));
    }

    #[test]
    fn test_delete_multiple_times() {
        let mut store = Store::new();
        store.insert("key".to_string(), "value".to_string()).unwrap();
        store.delete("key");
        let result = store.get("key");
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Key not found"));
    }
}
