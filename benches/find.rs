use criterion::{black_box, criterion_group, criterion_main, Criterion};
use futures::TryStreamExt;
use mongodb::bson::RawDocumentBuf;
use mongodb::{bson::doc, options::ClientOptions, Client};
use tokio::runtime::Runtime;

/// Benchmark find with RawDocumentBuf: using find vs find_raw
fn bench_find_raw(c: &mut Criterion) {
    dotenv::from_filename(".env.local").ok();
    dotenv::from_filename(".env.example").ok();

    let connection_string =
        std::env::var("MONGODB_HOST").unwrap_or_else(|_| "mongodb://mongodb:27018".to_string());

    let runtime = Runtime::new().unwrap();
    let client = runtime.block_on(async {
        let options = ClientOptions::parse(&connection_string).await.unwrap();
        Client::with_options(options).unwrap()
    });

    let db = client.database("test");
    let coll = db.collection::<RawDocumentBuf>("test_collection");

    let mut group = c.benchmark_group("find_raw_comparison");
    group.sample_size(10);

    // 1k documents
    group.bench_function("find_1k", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let results: Vec<RawDocumentBuf> = coll
                    .find(doc! {})
                    .batch_size(1024)
                    .limit(1_000)
                    .await
                    .unwrap()
                    .try_collect()
                    .await
                    .unwrap();
                black_box(results);
            });
        });
    });

    group.bench_function("find_raw_1k", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let results: Vec<RawDocumentBuf> = coll
                    .find_raw(doc! {})
                    .batch_size(1024)
                    .limit(1_000)
                    .await
                    .unwrap()
                    .try_collect()
                    .await
                    .unwrap();
                black_box(results);
            });
        });
    });

    // 10k documents
    group.bench_function("find_10k", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let results: Vec<RawDocumentBuf> = coll
                    .find(doc! {})
                    .batch_size(1024)
                    .limit(10_000)
                    .await
                    .unwrap()
                    .try_collect()
                    .await
                    .unwrap();
                black_box(results);
            });
        });
    });

    group.bench_function("find_raw_10k", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let results: Vec<RawDocumentBuf> = coll
                    .find_raw(doc! {})
                    .batch_size(1024)
                    .limit(10_000)
                    .await
                    .unwrap()
                    .try_collect()
                    .await
                    .unwrap();
                black_box(results);
            });
        });
    });

    // 100k documents
    group.bench_function("find_100k", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let results: Vec<RawDocumentBuf> = coll
                    .find(doc! {})
                    .batch_size(1024)
                    .limit(100_000)
                    .await
                    .unwrap()
                    .try_collect()
                    .await
                    .unwrap();
                black_box(results);
            });
        });
    });

    group.bench_function("find_raw_100k", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let results: Vec<RawDocumentBuf> = coll
                    .find_raw(doc! {})
                    .batch_size(1024)
                    .limit(100_000)
                    .await
                    .unwrap()
                    .try_collect()
                    .await
                    .unwrap();
                black_box(results);
            });
        });
    });

    group.finish();
}

/// Benchmark find_raw with 100k documents at different batch sizes
fn bench_find_raw_batch_sizes(c: &mut Criterion) {
    dotenv::from_filename(".env.local").ok();
    dotenv::from_filename(".env.example").ok();

    let connection_string =
        std::env::var("MONGODB_HOST").unwrap_or_else(|_| "mongodb://mongodb:27018".to_string());

    let runtime = Runtime::new().unwrap();
    let client = runtime.block_on(async {
        let options = ClientOptions::parse(&connection_string).await.unwrap();
        Client::with_options(options).unwrap()
    });

    let db = client.database("test");
    let coll = db.collection::<RawDocumentBuf>("test_collection");

    let mut group = c.benchmark_group("find_raw_batch_sizes");
    group.sample_size(10);

    // Batch size 101
    group.bench_function("batch_101", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let results: Vec<RawDocumentBuf> = coll
                    .find_raw(doc! {})
                    .batch_size(101)
                    .limit(100_000)
                    .await
                    .unwrap()
                    .try_collect()
                    .await
                    .unwrap();
                black_box(results);
            });
        });
    });

    // Batch size 512
    group.bench_function("batch_512", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let results: Vec<RawDocumentBuf> = coll
                    .find_raw(doc! {})
                    .batch_size(512)
                    .limit(100_000)
                    .await
                    .unwrap()
                    .try_collect()
                    .await
                    .unwrap();
                black_box(results);
            });
        });
    });

    // Batch size 1024
    group.bench_function("batch_1024", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let results: Vec<RawDocumentBuf> = coll
                    .find_raw(doc! {})
                    .batch_size(1024)
                    .limit(100_000)
                    .await
                    .unwrap()
                    .try_collect()
                    .await
                    .unwrap();
                black_box(results);
            });
        });
    });

    // Batch size 4096
    group.bench_function("batch_4096", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let results: Vec<RawDocumentBuf> = coll
                    .find_raw(doc! {})
                    .batch_size(4096)
                    .limit(100_000)
                    .await
                    .unwrap()
                    .try_collect()
                    .await
                    .unwrap();
                black_box(results);
            });
        });
    });

    // Batch size 10000
    group.bench_function("batch_10000", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let results: Vec<RawDocumentBuf> = coll
                    .find_raw(doc! {})
                    .batch_size(10000)
                    .limit(100_000)
                    .await
                    .unwrap()
                    .try_collect()
                    .await
                    .unwrap();
                black_box(results);
            });
        });
    });

    // Batch size 20000
    group.bench_function("batch_20000", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let results: Vec<RawDocumentBuf> = coll
                    .find_raw(doc! {})
                    .batch_size(10000)
                    .limit(100_000)
                    .await
                    .unwrap()
                    .try_collect()
                    .await
                    .unwrap();
                black_box(results);
            });
        });
    });

    group.finish();
}

criterion_group!(benches, bench_find_raw, bench_find_raw_batch_sizes);
criterion_main!(benches);
