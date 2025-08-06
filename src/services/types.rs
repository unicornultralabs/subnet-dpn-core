use serde::{Deserialize, Serialize};

use crate::types::connection::ProxyAccData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PeerChanged {
    Connected(PeerChangedInfo),
    Disconnected(PeerChangedInfo),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerChangedInfo {
    pub uuid: String,
    pub login_session_id: String,
    pub ip_u32: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProxyAccChanged {
    Created(ProxyAccData),
    Updated(ProxyAccData),
    Deleted(String), // proxy_acc_id
    RefreshAll(),
    DeletedAlgoProxyAcc(String), // algo_proxy_session_id
}
