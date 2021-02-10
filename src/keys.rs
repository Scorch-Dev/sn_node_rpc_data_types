
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// A struct to hold the result of get-keys
#[derive(Serialize, Deserialize, Debug)]
pub struct GetKeysResult {
    /// Node's id/public key as hex
    pub node_id: String,
    /// Node's reward key as hex
    pub reward_key: String,
}
