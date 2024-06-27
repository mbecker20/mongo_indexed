use mongo_indexed::doc;
use mongo_indexed_derive::MongoIndexed;
use mongodb::{bson::oid::ObjectId, options::ClientOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, MongoIndexed)]
#[unique_doc_index({ "username": 1, "email": 1 })]
#[collection_name(users)] // By default, this will be the name of the struct it's defined on, in this case 'User'.
pub struct User {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[index]
  pub username: String,

  #[index]
  pub email: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // instantiate your mongo client
  let mongo =
    mongodb::Client::with_options(ClientOptions::parse("mongodb://localhost:27017").await?)?;
  let db = mongo.database("my-db");

  // Since index calls are a no-op if the indexes do not change, your APIs can
  // safely use create_indexes = true even when restarting all the time.
  // Just be careful if the indexes change and the collection is large.
  let create_indexes = true;

  // will return a handle to 'users' collection on 'my-db', which has the specified indexes created.
  let users = mongo_indexed::collection::<User>(&db, create_indexes).await?;

  let user = users.find_one(doc! { "username": "mogh" }).await?;
  println!("{user:?}");

  Ok(())
}
