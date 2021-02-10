
use serde::{Deserialize, Serialize};

/// A struct to hold the result of get-keys
#[derive(Serialize, Deserialize)]
pub struct GetKeysResult {
    /// Node's id/public key as hex
    pub node_id: String,
    /// Node's reward key as hex
    pub reward_key: String,
}
