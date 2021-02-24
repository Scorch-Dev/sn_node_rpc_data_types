
use std::{
    path::PathBuf,
    net::IpAddr,
};
use sn_data_types::PublicKey;

pub struct GetRoutingInfoResult {
    /// Node id
    node_id: PublicKey,
    /// age of the node
    age: u8,
    /// External port of the node
    local_port: u16,
    /// External IP address of the node
    local_ip_addr: IpAddr,
}