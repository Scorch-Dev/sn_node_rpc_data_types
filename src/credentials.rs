
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Used to pass credentials to issue the rpc
#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    /// One time passphrase for the RPC
    pub passphrase: String,
    /// Signature of the passphrase as hex
    pub signature: String,
}