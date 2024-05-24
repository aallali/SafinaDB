use criterion::{criterion_group, criterion_main, Criterion};
#[path = "../src/kv_store.rs"] mod kv_store;
use kv_store::Store;

pub fn bench_insert(c: &mut Criterion) {
    println!("Running bench_insert");
    c.bench_function("insert 1000 keys", |b| {
        b.iter(|| {
        let mut store = Store::new();
            for i in 0..1000 {
                store.insert(format!("key-{}", i), "value".to_string()).unwrap();
            }
        })
    });
}

pub fn bench_get(c: &mut Criterion) {
    println!("Running bench_get");
    c.bench_function("get 1000 keys", |b| {
        let mut store = Store::new();
        for i in 0..1000 {
            store.insert(format!("key{}", i), "value".to_string()).unwrap();
        }
        b.iter(|| {
            for i in 0..1000 {
                let _ = store.get(&format!("key{}", i));
            }
        })
    });
}

pub fn bench_update(c: &mut Criterion) {
    c.bench_function("update 1000 keys", |b| {
        let mut store = Store::new();
        for i in 0..1000 {
            store.insert(format!("key{}", i), "value".to_string()).unwrap();
        }
        b.iter(|| {
            for i in 0..1000 {
                store.update(&format!("key{}", i), "new_value").unwrap();
            }
        })
    });
}

pub fn bench_delete(c: &mut Criterion) {
    c.bench_function("delete 1000 keys", |b| {
        let mut store = Store::new();
        for i in 0..1000 {
            store.insert(format!("key{}", i), "value".to_string()).unwrap();
        }
        b.iter(|| {
            for i in 0..1000 {
                store.delete(&format!("key{}", i));
            }
        })
    });
}

criterion_group!(benches, bench_insert, bench_get, bench_update, bench_delete);
criterion_main!(benches);
