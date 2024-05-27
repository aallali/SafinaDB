mod cli;
mod kv_store;
mod storage;

use kv_store::Store;

use once_cell::sync::Lazy;
use std::sync::Mutex;

/// A globally accessible, thread-safe, lazily initialized instance of `Store`.
///
/// # Details
/// - `static STORE`: Declares a static variable named `STORE` that is globally accessible.
/// - `Lazy<Mutex<Store>>`: Uses the `Lazy` type from the `once_cell` crate to defer the initialization
///   of the `Store` instance until it is first accessed. The `Mutex` ensures that access to the `Store`
///   is thread-safe by allowing only one thread to access it at a time.
/// - `Lazy::new(|| Mutex::new(Store::new()))`: This line initializes the `STORE` variable using a closure
///   (`||` syntax). The closure creates a new `Mutex` which wraps a new instance of `Store`. The `Lazy::new`
///   function ensures that this initialization happens only once, no matter how many times `STORE` is accessed.
///
/// # Example
/// ```rust
/// // Accessing the global STORE instance
/// let mut store = STORE.lock().unwrap();
/// store.insert(String::from("key"), String::from("value"));
/// ```
static STORE: Lazy<Mutex<Store>> = Lazy::new(|| Mutex::new(Store::new()));

fn main() -> Result<(), String> {
    {
        let mut store = STORE.lock().unwrap();
        println!("- Loading data...");
        match store.storage.load_file(Some("db")) {
            Ok(data) => store.data = data,
            Err(err) => {
                println!("Invalid: {}", err);
            }
        }
        println!("- Data overview:");
        for d in &store.data {
            println!("      - \"{}\" : \"{}\"", d.key, d.value)
        }
    }

    cli::run().unwrap();
    return Ok(());
}
