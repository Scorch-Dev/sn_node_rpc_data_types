use serde::{Deserialize, Serialize};
use sn_data_types::PublicKey;
use std::fmt::Debug;

/// Get info regarding rewards, like our reward-key
pub const METHOD_GET_REWARDS_INFO: &str = "get-rewards-info";

/// Used to set the reward key for the node
pub const METHOD_SET_REWARD_KEY: &str = "set-reward-key";

/// A struct to hold the result of get-reward-key
#[derive(Serialize, Deserialize, Debug)]
pub struct GetRewardsInfoResult {
    /// Reward key associated with this node id as hex
    pub reward_key: PublicKey,
    // TODO: Other info. Current balance, farming stats, etc.
}

/// What you get back after succesfully setting the reward key
#[derive(Serialize, Deserialize, Debug)]
pub struct SetRewardKeyResult {
    /// the old key you were using
    pub old_reward_key: PublicKey,
}

/// A struct to hold the parameters of set-reward-key
#[derive(Serialize, Deserialize, Debug)]
pub struct SetRewardKeyParams {
    /// The new reward key to use as hex
    pub reward_key: PublicKey,
}
