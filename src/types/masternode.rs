use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::geo::Geo;

pub const PEER_V0: [u8; 2] = [0u8, 0u8];
pub const PEER_V1: [u8; 2] = [0u8, 1u8];

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MasternodeInfo {
    pub peer_bind: String,
    pub client_bind: String,
    pub control_bind: String,
    pub web_bind: String,
    pub root_ca: Option<String>,
    pub geo: Geo,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AssignMasternodeRes {
    pub masternode: Option<MasternodeInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct ActivePeersClients {
    pub active_peers: u32,
    pub active_clients: u32,
}
