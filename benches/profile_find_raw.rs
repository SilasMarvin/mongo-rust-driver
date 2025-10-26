use futures::TryStreamExt;
use mongodb::bson::RawDocumentBuf;
use mongodb::{bson::doc, options::ClientOptions, Client};

#[tokio::main]
async fn main() {
    dotenv::from_filename(".env.local").ok();

    let connection_string =
        std::env::var("MONGODB_HOST").unwrap_or_else(|_| "mongodb://mongodb:27018".to_string());

    let options = ClientOptions::parse(&connection_string).await.unwrap();
    let client = Client::with_options(options).unwrap();

    let db = client.database("test");
    let coll = db.collection::<RawDocumentBuf>("test_collection");

    // Run find_raw multiple times to get good profiling data
    for _ in 0..10 {
        let _: Vec<RawDocumentBuf> = coll
            .find_raw(doc! {})
            .limit(100_000)
            .batch_size(100_000)
            .await
            .unwrap()
            .try_collect()
            .await
            .unwrap();
    }

    println!("Profiling complete!");
}
