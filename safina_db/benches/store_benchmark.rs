use criterion::{criterion_group, criterion_main, Criterion};
use safina_db::{Store, STORAGE_MUTEX};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn setup_test_store(db_name: &str) -> Store {
    let mut store = Store::new();
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let timestamp = since_the_epoch.as_secs();
    let db_name = format!("db-test-bench-{}-{}", db_name, timestamp);
    match STORAGE_MUTEX
        .lock()
        .unwrap()
        .load_file(Some(db_name.as_str()))
    {
        Ok(data) => store.data = data,
        Err(err) => {
            println!("Invalid: {}", err);
        }
    }
    store
}

pub fn bench_store_crud(c: &mut Criterion) {
    let mut group = c.benchmark_group("[Store persistent CRUD benchamrks");
    group.sample_size(10);
    let mut store = setup_test_store("all");
    let mut counter = 0;

    group.bench_function("insert 1000 keys", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let fresh_key = format!("b-{}-key-{}", counter, i);
                store
                    .insert(
                        &fresh_key,
                        &format!("value-{}", i),
                    )
                    .unwrap();
            }
            counter += 1;
        })
    });
    counter = 0;
    group.bench_function("get 1000 keys", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let fresh_key = format!("b-{}-key-{}", counter, i);
                let _ = store.get(&fresh_key);
            }
            counter += 1;
        })
    });
    counter = 0;
    group.bench_function("update 1000 keys", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let fresh_key = format!("b-{}-key-{}", counter, i);
                store.update(&fresh_key, "new_value").unwrap();
            }
            counter += 1;
        })
    });
    counter = 0;
    group.bench_function("delete 1000 keys", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let fresh_key = format!("b-{}-key-{}", counter, i);
                store.delete(&fresh_key);
            }
            counter += 1;
        })
    });
}

criterion_group!(benches, bench_store_crud);
criterion_main!(benches);
