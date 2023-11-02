use async_trait::async_trait;
use mongodb::{Collection, Database};
use serde::{de::DeserializeOwned, Serialize};

pub use mongo_indexed_derive as derive;
pub use mongodb::bson::Document;

mod helpers;

#[async_trait]
pub trait Indexed: Serialize + DeserializeOwned + Sync {
    fn default_collection_name() -> &'static str {
        ""
    }
    fn indexes() -> &'static [&'static str] {
        &[]
    }
    fn unique_indexes() -> &'static [&'static str] {
        &[]
    }
    fn sparse_indexes() -> &'static [&'static str] {
        &[]
    }
    fn doc_indexes() -> Vec<Document> {
        Vec::new()
    }
    fn unique_doc_indexes() -> Vec<Document> {
        Vec::new()
    }
    fn sparse_doc_indexes() -> Vec<Document> {
        Vec::new()
    }
    async fn collection(db: &Database, create_index: bool) -> anyhow::Result<Collection<Self>> {
        Self::collection_with_name(db, Self::default_collection_name(), create_index).await
    }
    async fn collection_with_name(
        db: &mongodb::Database,
        coll_name: &str,
        create_index: bool,
    ) -> anyhow::Result<Collection<Self>> {
        let coll = db.collection(coll_name);

        if create_index {
            for index in Self::indexes() {
                helpers::create_index(&coll, index).await?;
            }
            for unique_index in Self::unique_indexes() {
                helpers::create_unique_index(&coll, unique_index).await?;
            }
            for sparse_index in Self::sparse_indexes() {
                helpers::create_sparse_index(&coll, sparse_index).await?;
            }
            for doc_index in Self::doc_indexes() {
                helpers::create_index_from_doc(&coll, doc_index).await?;
            }
            for unique_doc_index in Self::unique_doc_indexes() {
                helpers::create_unique_index_from_doc(&coll, unique_doc_index).await?;
            }
            for sparse_doc_index in Self::sparse_doc_indexes() {
                helpers::create_sparse_index_from_doc(&coll, sparse_doc_index).await?;
            }
        }
        Ok(coll)
    }
}
