use serde::{Deserialize, Serialize};
use std::{fmt::Debug, path::PathBuf};

/// Get info related to storage on this node (e.g. used vs total offered)
pub const METHOD_GET_STORAGE_INFO: &str = "get-storage-info";

/// Result of a get-storage-info query
#[derive(Serialize, Deserialize, Debug)]
pub struct GetStorageInfoResult {
    /// Root dir of the node itself
    pub node_root: PathBuf,
    /// Total used storage amount in bytes
    pub used: u64,
    /// Total storage being offered in bytes
    pub total: u64,
    /// Pairs of root path and value for local stores
    /// that comprise the used amount
    pub local_stores: Vec<(PathBuf, u64)>,
}
