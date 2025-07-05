use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerSpeedTestRespond {
    pub version: String,
    pub throughput: f64,
    pub jitter: u128,
    pub packet_loss: f64,
    pub connected_at: u64,
    pub updated_at: u64,
    pub total_test_performed: u32,
    pub peer_ip_v4: String,
}
