use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Gets a specified log
pub const METHOD_GET_LOGS: &str = "get-logs";

/// Id for the plain-text logs
pub const LOG_ID_PLAINTEXT: u64 = 0;

/// Parameters required to specify fetching of log entries
/// Used to fetch a up to a up to a specified number of log lines
/// [start_idx : start_idx + num_lines).
/// If num_lines overflows the end of the array, then
/// the return will contain less than num_lines entries
#[derive(Serialize, Deserialize, Debug)]
pub struct GetLogsParams {
    /// ID of the log to get
    /// Currently, there is only one log (the text logs)
    /// But in future it is conceivable to have various logs
    /// being collected (e.g. logs made of structured entries
    /// rather than Strings)
    pub log_id: u64,
    /// starting log index from which to fetch num_lines
    /// start_idx >= 0 indicates normal indexing
    /// start_idx < 0 means start fetching logs at index
    /// (total_num_log_lines + start_idx).
    pub start_idx: i64,
    /// Maximum number of lines to fetch
    pub num_lines: u64,
}

/// The result of a get-logs call
#[derive(Serialize, Deserialize, Debug)]
pub struct GetLogsResult {
    /// The log lines as strings.
    /// To encode structured data, one can always convert
    /// it to hex a hex string
    pub lines: Vec<String>,
}
