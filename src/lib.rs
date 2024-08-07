use mongodb::{
  options::IndexOptions, results::CreateIndexResult, Collection, Database, IndexModel,
};
use serde::{de::DeserializeOwned, Serialize};

pub use mongo_indexed_derive as derive;
pub use mongodb::bson::{doc, Document};

pub trait Indexed: Serialize + DeserializeOwned + Send + Sync {
  fn default_collection_name() -> &'static str;
  fn indexes() -> &'static [&'static str];
  fn unique_indexes() -> &'static [&'static str];
  fn sparse_indexes() -> &'static [&'static str];
  fn doc_indexes() -> Vec<Document>;
  fn unique_doc_indexes() -> Vec<Document>;
  fn sparse_doc_indexes() -> Vec<Document>;
}

pub async fn collection<T: Indexed>(
  db: &Database,
  should_create_index: bool,
) -> mongodb::error::Result<Collection<T>> {
  collection_with_name::<T>(db, T::default_collection_name(), should_create_index).await
}

pub async fn collection_with_name<T: Indexed>(
  db: &Database,
  coll_name: &str,
  should_create_index: bool,
) -> mongodb::error::Result<Collection<T>> {
  let coll = db.collection(coll_name);

  if should_create_index {
    for index in T::indexes() {
      create_index(&coll, index).await?;
    }
    for unique_index in T::unique_indexes() {
      create_unique_index(&coll, unique_index).await?;
    }
    for sparse_index in T::sparse_indexes() {
      create_sparse_index(&coll, sparse_index).await?;
    }
    for doc_index in T::doc_indexes() {
      create_index_from_doc(&coll, doc_index).await?;
    }
    for unique_doc_index in T::unique_doc_indexes() {
      create_unique_index_from_doc(&coll, unique_doc_index).await?;
    }
    for sparse_doc_index in T::sparse_doc_indexes() {
      create_sparse_index_from_doc(&coll, sparse_doc_index).await?;
    }
  }
  Ok(coll)
}

pub async fn create_index<T: Send + Sync>(
  collection: &Collection<T>,
  field: &str,
) -> mongodb::error::Result<CreateIndexResult> {
  let index = IndexModel::builder().keys(doc! { field: 1 }).build();
  collection.create_index(index).await
}

pub async fn create_unique_index<T: Send + Sync>(
  collection: &Collection<T>,
  field: &str,
) -> mongodb::error::Result<CreateIndexResult> {
  let options = IndexOptions::builder().unique(true).build();
  let index = IndexModel::builder()
    .keys(doc! { field: 1 })
    .options(options)
    .build();
  collection.create_index(index).await
}

pub async fn create_sparse_index<T: Send + Sync>(
  collection: &Collection<T>,
  field: &str,
) -> mongodb::error::Result<CreateIndexResult> {
  let options = IndexOptions::builder().sparse(true).build();
  let index = IndexModel::builder()
    .keys(doc! { field: 1 })
    .options(options)
    .build();
  collection.create_index(index).await
}

pub async fn create_index_from_doc<T: Send + Sync>(
  collection: &Collection<T>,
  index_doc: Document,
) -> mongodb::error::Result<CreateIndexResult> {
  let index = IndexModel::builder().keys(index_doc.clone()).build();
  collection.create_index(index).await
}

pub async fn create_unique_index_from_doc<T: Send + Sync>(
  collection: &Collection<T>,
  index_doc: Document,
) -> mongodb::error::Result<CreateIndexResult> {
  let options = IndexOptions::builder().unique(true).build();
  let index = IndexModel::builder()
    .keys(index_doc.clone())
    .options(options)
    .build();
  collection.create_index(index).await
}

pub async fn create_sparse_index_from_doc<T: Send + Sync>(
  collection: &Collection<T>,
  index_doc: Document,
) -> mongodb::error::Result<CreateIndexResult> {
  let options = IndexOptions::builder().sparse(true).build();
  let index = IndexModel::builder()
    .keys(index_doc.clone())
    .options(options)
    .build();
  collection.create_index(index).await
}
