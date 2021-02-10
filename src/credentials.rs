
use serde::{Deserialize, Serialize};

/// Used to pass credentials to issue the rpc
#[derive(Serialize, Deserialize)]
pub struct Credentials {
    /// One time passphrase for the RPC
    passphrase: String,
    /// Signature of the passphrase as hex
    signature: String,
}