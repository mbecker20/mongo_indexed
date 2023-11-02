## Configure mongo indexing right on your rust structs

### Example

```rust
use serde::{Serialize, Deserialize};
use mongo_indexed::{derive::MongoIndexed, Indexed};
use mongodb::bson::doc;

#[derive(Serialize, Deserialize, MongoIndexed)]
#[unique_doc_index(doc! { "username": 1, "email": 1 })]
pub struct User {
  #[serde(rename = "_id")]
  pub id: ObjectId,
  
  #[index]
  pub username: String,

  #[index]
  pub email: String,
}

// Use the collection initializer

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let mongo = ... // init mongodb::Client

  let create_index = true;

  let users = User::collection(&mongo, "db_name", create_index).await?; // mongodb::Collection<User>

  // use the indexed collection...

  Ok(())
}
```