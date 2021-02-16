use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Get info related to storage on this node (e.g. used vs total offered)
pub const METHOD_GET_STORAGE_INFO: &str = "get-storage-info";

/// Result of a get-storage-info query
#[derive(Serialize, Deserialize, Debug)]
pub struct GetStorageInfoResult {
    /// Total used storage amount in bytes
    pub used: u64,
    /// Total storage being offered in bytes
    pub total: u64,
}
