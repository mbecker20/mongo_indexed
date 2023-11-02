use anyhow::Context;
use mongodb::{
    bson::{doc, Document},
    options::IndexOptions,
    results::CreateIndexResult,
    Collection, IndexModel,
};

pub async fn create_index<T>(
    collection: &Collection<T>,
    field: &str,
) -> anyhow::Result<CreateIndexResult> {
    let index = IndexModel::builder().keys(doc! { field: 1 }).build();
    collection
        .create_index(index, None)
        .await
        .context(format!("failed to create index on {field}"))
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
        .context(format!("failed to create index on {field}"))
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
        .context(format!("failed to create index on {field}"))
}

pub async fn create_index_from_doc<T>(
    collection: &Collection<T>,
    index_doc: Document,
) -> anyhow::Result<CreateIndexResult> {
    let index = IndexModel::builder().keys(index_doc).build();
    collection
        .create_index(index, None)
        .await
        .context("failed to create doc index")
}

pub async fn create_unique_index_from_doc<T>(
    collection: &Collection<T>,
    index_doc: Document,
) -> anyhow::Result<CreateIndexResult> {
    let options = IndexOptions::builder().unique(true).build();
    let index = IndexModel::builder()
        .keys(index_doc)
        .options(options)
        .build();
    collection
        .create_index(index, None)
        .await
        .context("failed to create unique doc index")
}

pub async fn create_sparse_index_from_doc<T>(
    collection: &Collection<T>,
    index_doc: Document,
) -> anyhow::Result<CreateIndexResult> {
    let options = IndexOptions::builder().sparse(true).build();
    let index = IndexModel::builder()
        .keys(index_doc)
        .options(options)
        .build();
    collection
        .create_index(index, None)
        .await
        .context("failed to create sparse doc index")
}
