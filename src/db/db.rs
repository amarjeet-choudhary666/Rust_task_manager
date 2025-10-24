use mongodb::{Client, bson::doc, options::ClientOptions, error::Result};

pub async fn init_db(mongo_uri: String) -> Result<Client> {
    let client_options = ClientOptions::parse(&mongo_uri).await?;

    let client = Client::with_options(client_options)?;

    client
        .database("admin")
        .run_command(doc! { "ping": 1 })
        .await?;

    println!("✅ Connected to MongoDB");

    Ok(client)
}
