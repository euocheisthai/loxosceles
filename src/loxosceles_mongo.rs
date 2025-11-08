use crate::models::LoxoUserConfig;
use mongodb::{Client, Database, options::ClientOptions};
use std::env;

pub async fn establish_mongo_connection() -> mongodb::error::Result<(Client, Database)> {
    let mongodb_cluster_url = env::var("MONGODB_HOST").expect("MONGODB_HOST env var should be specified");
    let mongo_username = env::var("MONGODB_USER").expect("MONGODB_USER env var should be specified");
    let mongo_password = env::var("MONGODB_PASSWORD").expect("MONGODB_PASSWORD env var should be specified");
    
    // MONGODB_DB == test_db
    // TODO: make 2 contours with diff. DBs each
    let main_database = env::var("MONGODB_DB").expect("MONGODB_DB env var should be specified");

    let client_options = ClientOptions::parse(
        format!("mongodb://{}:{}@{}/{}", mongo_username, mongo_password, mongodb_cluster_url, main_database),
    )
    .await?;

    let mongo_client = Client::with_options(client_options)?;
    let _client_session: mongodb::action::StartSession<'_> = mongo_client.start_session();
    let mongo_database = mongo_client.database(&main_database);

    Ok((mongo_client, mongo_database))
}

pub async fn prepare_mongo(mongo_database: Database) -> () {
    // create collection(s) here
    // at least 1 collection with LoxoUserConfig model
    // what do i return? idk
    // https://docs.rs/mongodb/latest/mongodb/struct.Collection.html
     
    let coll_loxo_user_configs = mongo_database.collection::<LoxoUserConfig>("loxo_user_configs");
    // coll_loxo_user_configs.clone().insert_one(LoxoUserConfig { id: i }).await;

}

pub async fn write_to_collection() -> () {
}