use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// methods
pub const METHOD_GET_STORAGE_INFO: &str = "get-storage-info";

/// Result of a get-storage-info query
#[derive(Serialize, Deserialize, Debug)]
pub struct GetStorageInfoResult {
    /// Total used storage amount
    pub used: u64,
    /// Total storage being offered
    pub total: u64,
}
