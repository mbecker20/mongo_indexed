use std::{collections::HashMap, hash::Hash, path::PathBuf};

use async_trait::async_trait;
use mongodb::{bson::oid::ObjectId, Collection};
use serde::{de::DeserializeOwned, Serialize};

pub use mongo_indexed_derive as derive;
pub use mongodb::bson::Document;

mod helpers;

#[async_trait]
pub trait Indexed: Serialize + DeserializeOwned + Sync {
    fn default_collection_name() -> &'static str {
        ""
    }
    fn indexes() -> Vec<String> {
        Vec::new()
    }
    fn unique_indexes() -> Vec<String> {
        Vec::new()
    }
    fn sparse_indexes() -> Vec<String> {
        Vec::new()
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
    async fn collection(
        mongo: &mongodb::Client,
        db_name: &str,
        create_index: bool,
    ) -> anyhow::Result<Collection<Self>> {
        Self::collection_with_custom_name(
            mongo,
            db_name,
            Self::default_collection_name(),
            create_index,
        )
        .await
    }
    async fn collection_with_custom_name(
        mongo: &mongodb::Client,
        db_name: &str,
        coll_name: &str,
        create_index: bool,
    ) -> anyhow::Result<Collection<Self>> {
        let coll = mongo.database(db_name).collection(coll_name);

        if create_index {
            for index in Self::indexes() {
                helpers::create_index(&coll, &index).await?;
            }
            for unique_index in Self::unique_indexes() {
                helpers::create_unique_index(&coll, &unique_index).await?;
            }
            for sparse_index in Self::sparse_indexes() {
                helpers::create_sparse_index(&coll, &sparse_index).await?;
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

macro_rules! impl_indexed_basic {
    ($($ty:ty),*) => {
        $(impl Indexed for $ty {})*
        $(impl Indexed for Option<$ty> {})*
    };
}

macro_rules! impl_indexed_nested {
    ($($ty:ty),*) => {
        $(impl<T: Serialize + DeserializeOwned + Sync> Indexed for $ty {})*
        $(impl<T: Serialize + DeserializeOwned + Sync> Indexed for Option<$ty> {})*
    };
}

impl_indexed_basic!(
    String, PathBuf, bool, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, f32, f64,
    isize, ObjectId, Document
);
impl_indexed_nested!(Vec<T>);

impl<
        K: Serialize + DeserializeOwned + Sync + Eq + Hash,
        V: Serialize + DeserializeOwned + Sync,
    > Indexed for HashMap<K, V>
{
}
