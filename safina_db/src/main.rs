use safina_db::{cli, STORAGE_MUTEX, STORE_MUTEX};


fn main() -> Result<(), String> {
    {
        let mut store = STORE_MUTEX.lock().unwrap();
        println!("- Loading data...");
        match STORAGE_MUTEX.lock().unwrap().load_file(Some("db")) {
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
