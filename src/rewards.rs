use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// A struct to hold the result of get-reward-key
#[derive(Serialize, Deserialize, Debug)]
pub struct GetRewardsInfoResult {
    /// Reward key associated with this node id as hex
    pub reward_key: String,
    // TODO: Other info. Current balance, farming stats, etc.
}

/// set reward key result is the same as get reward key result
pub type SetRewardKeyResult = GetRewardsInfoResult;

/// A struct to hold the parameters of set-reward-key
#[derive(Serialize, Deserialize, Debug)]
pub struct SetRewardKeyParams {
    /// The new reward key to use as hex
    pub reward_key: String,
}
