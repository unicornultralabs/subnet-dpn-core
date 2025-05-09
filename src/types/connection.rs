use dpn_proto::proxy_acc::ProtoProxyAcc;
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use bitcode::{Encode, Decode};
use crate::utils::{bytes_to_hex_string, hash::hash};

pub const DEFAULT_IP_ROTATION_PERIOD: i64 = 300;
pub const MAX_INACTIVE_TIME: i64 = 300; // 300 seconds

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserConnectStats {
    pub user_addr: String,
    pub last_connect_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionOverviewV2 {
    pub total_bandwidth_served: i64,
    pub total_rewards: i64,
    pub total_rewards_v2: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PeernodeInfo {
    pub peer_id: String,
    pub ip_addr: String,
    pub throughput: f64,
    pub rate_per_kb: u64,
    pub rate_per_second: u64,
    pub city_geoname_id: u32,
    pub country_geoname_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct PeerStats {
    pub masternode_id: String,
    pub session_hash: String,
    pub download: u64,
    pub upload: u64,
    pub c_download: u64,
    pub c_upload: u64,
    pub login_session_id: String,
}

#[derive(Debug, Clone, FromPrimitive, Serialize, Deserialize, ToSchema)]
pub enum PrioritizedIPLevel {
    /// Replacable by other IPs if prioritized IP is unavailable
    Normal,
    /// Always use prioritized IP even if it is unavailable
    Strict,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct ProxyAccData {
    pub id: String,
    pub password: String,
    pub ip_rotation_period: i64,
    pub whitelisted_ip: Option<String>,
    pub user_addr: String,
    pub country_geoname_id: i64,
    pub city_geoname_id: Option<i64>,
    pub rate_per_kb: i64,
    pub rate_per_second: i64,
    pub prioritized_ip: Option<String>,
    pub prioritized_ip_level: Option<PrioritizedIPLevel>,
    pub created_at: i64,
}

impl ProxyAccData {
    pub fn new(
        password: String,
        ip_rotation_period: i64,
        whitelisted_ip: Option<String>,
        user_addr: String,
        country_geoname_id: i64,
        city_geoname_id: Option<i64>,
        rate_per_kb: i64,
        rate_per_second: i64,
        prioritized_ip: Option<String>,
        prioritized_ip_level: Option<PrioritizedIPLevel>,
        created_at: i64,
    ) -> Self {
        let mut _self = Self {
            id: "".to_string(),
            password,
            ip_rotation_period,
            whitelisted_ip,
            user_addr,
            country_geoname_id,
            city_geoname_id,
            rate_per_kb,
            rate_per_second,
            prioritized_ip,
            prioritized_ip_level,
            created_at,
        };

        let proto: ProtoProxyAcc = _self.clone().into();
        let binding = ::prost::Message::encode_to_vec(&proto);
        let bz = binding.as_slice();

        _self.id = bytes_to_hex_string(hash(bz).as_bytes());
        _self
    }
}

impl Into<ProtoProxyAcc> for ProxyAccData {
    fn into(self) -> ProtoProxyAcc {
        ProtoProxyAcc {
            user_addr: self.user_addr,
            created_at: self.created_at,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub enum VerifyProxyAccData {
    // ip
    IP(String),
    // username, password
    BasicAuth(String, String),
}
