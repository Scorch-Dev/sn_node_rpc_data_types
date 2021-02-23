use serde::{Deserialize, Serialize};
use serde_with::{hex::Hex, As};
use sn_data_types::Signature;
use std::fmt::Debug;

/// Used to pass credentials to issue the rpc
#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    /// One time passphrase for the RPC
    #[serde(with = "As::<Hex>")]
    pub passphrase: Vec<u8>,
    /// Signature of the passphrase as hex
    pub signature: Signature,
}
