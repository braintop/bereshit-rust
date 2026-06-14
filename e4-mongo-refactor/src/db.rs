use std::env;

use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};

/// Read MONGO_URI from the environment, connect to the cluster, ping it,
/// and return a ready-to-use client.
pub async fn connect() -> mongodb::error::Result<Client> {
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");

    let mut client_options = ClientOptions::parse(mongo_uri).await?;

    let server_api = ServerApi::builder()
        .version(ServerApiVersion::V1)
        .build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options)?;

    client
        .database("cluster0")
        .run_command(doc! { "ping": 1 })
        .await?;

    println!("Connected to MongoDB");

    Ok(client)
}
