#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("failed to create index | index: {index} | error: {err:?}")]
  IndexFailed {
    index: &'static str,
    err: mongodb::error::Error,
  },
  #[error("failed to create unique index | index: {index} | error: {err:?}")]
  UniqueIndexFailed {
    index: &'static str,
    err: mongodb::error::Error,
  },
  #[error("failed to create sparse index | index: {index} | error: {err:?}")]
  SparseIndexFailed {
    index: &'static str,
    err: mongodb::error::Error,
  },
  #[error("failed to create doc index | error: {err:?}")]
  DocIndexFailed { err: mongodb::error::Error },
  #[error("failed to create unique doc index | error: {err:?}")]
  UniqueDocIndexFailed { err: mongodb::error::Error },
  #[error("failed to create sparse doc index | error: {err:?}")]
  SparseDocIndexFailed { err: mongodb::error::Error },
}
