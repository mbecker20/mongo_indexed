use anyhow::Context;
use async_trait::async_trait;
use mongodb::{
    bson::{doc, Document},
    options::IndexOptions,
    results::CreateIndexResult,
    Collection, Database, IndexModel,
};
use serde::{de::DeserializeOwned, Serialize};

pub use mongo_indexed_derive as derive;
pub use mongodb::bson;

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
        should_create_index: bool,
    ) -> anyhow::Result<Collection<Self>> {
        let coll = db.collection(coll_name);

        if should_create_index {
            for index in Self::indexes() {
                create_index(&coll, index).await?;
            }
            for unique_index in Self::unique_indexes() {
                create_unique_index(&coll, unique_index).await?;
            }
            for sparse_index in Self::sparse_indexes() {
                create_sparse_index(&coll, sparse_index).await?;
            }
            for doc_index in Self::doc_indexes() {
                create_index_from_doc(&coll, doc_index).await?;
            }
            for unique_doc_index in Self::unique_doc_indexes() {
                create_unique_index_from_doc(&coll, unique_doc_index).await?;
            }
            for sparse_doc_index in Self::sparse_doc_indexes() {
                create_sparse_index_from_doc(&coll, sparse_doc_index).await?;
            }
        }
        Ok(coll)
    }
}

pub async fn create_index<T>(
    collection: &Collection<T>,
    field: &str,
) -> anyhow::Result<CreateIndexResult> {
    let index = IndexModel::builder().keys(doc! { field: 1 }).build();
    collection
        .create_index(index, None)
        .await
        .with_context(|| format!("failed to create index on {field}"))
}

pub async fn create_unique_index<T>(
    collection: &Collection<T>,
    field: &str,
) -> anyhow::Result<CreateIndexResult> {
    let options = IndexOptions::builder().unique(true).build();
    let index = IndexModel::builder()
        .keys(doc! { field: 1 })
        .options(options)
        .build();
    collection
        .create_index(index, None)
        .await
        .with_context(|| format!("failed to create index on {field}"))
}

pub async fn create_sparse_index<T>(
    collection: &Collection<T>,
    field: &str,
) -> anyhow::Result<CreateIndexResult> {
    let options = IndexOptions::builder().sparse(true).build();
    let index = IndexModel::builder()
        .keys(doc! { field: 1 })
        .options(options)
        .build();
    collection
        .create_index(index, None)
        .await
        .with_context(|| format!("failed to create index on {field}"))
}

pub async fn create_index_from_doc<T>(
    collection: &Collection<T>,
    index_doc: Document,
) -> anyhow::Result<CreateIndexResult> {
    let index = IndexModel::builder().keys(index_doc.clone()).build();
    collection
        .create_index(index, None)
        .await
        .with_context(|| format!("failed to create index from {index_doc:?}"))
}

pub async fn create_unique_index_from_doc<T>(
    collection: &Collection<T>,
    index_doc: Document,
) -> anyhow::Result<CreateIndexResult> {
    let options = IndexOptions::builder().unique(true).build();
    let index = IndexModel::builder()
        .keys(index_doc.clone())
        .options(options)
        .build();
    collection
        .create_index(index, None)
        .await
        .with_context(|| format!("failed to create unique index from {index_doc:?}"))
}

pub async fn create_sparse_index_from_doc<T>(
    collection: &Collection<T>,
    index_doc: Document,
) -> anyhow::Result<CreateIndexResult> {
    let options = IndexOptions::builder().sparse(true).build();
    let index = IndexModel::builder()
        .keys(index_doc.clone())
        .options(options)
        .build();
    collection
        .create_index(index, None)
        .await
        .with_context(|| format!("failed to create sparse index from {index_doc:?}"))
}
