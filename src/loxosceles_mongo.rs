use mongodb::{options::ClientOptions, Client};
use std::env;

pub async fn establish_mongo_connection() -> mongodb::error::Result<()> {
    let mongodb_cluster_url = env::var("MONGODB_HOST").expect("MONGODB_HOST env var should be specified");
    let mongo_username = env::var("MONGODB_USER").expect("MONGODB_USER env var should be specified");
    let mongo_password = env::var("MONGODB_PASSWORD").expect("MONGODB_PASSWORD env var should be specified");
    
    // MONGODB_DB == test_db
    let demo_database = env::var("MONGODB_DB").expect("MONGODB_DB env var should be specified");

    let client_options = ClientOptions::parse(
        format!("mongodb://{}:{}@{}/{}", mongo_username, mongo_password, mongodb_cluster_url, demo_database),
    )
    .await?;

    let mongo_client = Client::with_options(client_options)?;
    let _database = mongo_client.database(&demo_database);

    Ok(())
}