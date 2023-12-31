use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MasternodeInfo {
    pub region: String,
    pub peer_bind: String,
    pub client_bind: String,
    pub control_bind: String,
    pub web_bind: String,
    pub root_ca: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AssignMasternodeRes {
    pub masternode: Option<MasternodeInfo>,
}
