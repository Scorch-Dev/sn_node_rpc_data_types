
use serde::{Deserialize, Serialize};

/// A struct to hold the result of get-keys
#[derive(Serialize, Deserialize)]
struct GetKeysResult {
    /// Node's id/public key as hex
    node_id: String,
    /// Node's reward key as hex
    reward_key: String,
}
