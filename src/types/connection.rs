use dpn_proto::proxy_acc::ProtoProxyAcc;
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use::bitcode::{Decode, Encode};
use crate::{types::proxy_types::{BanStatus, ProxyStatus, ProxyType}, utils::{bytes_to_hex_string, hash::hash}};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelisted_ip: Option<String>,
    pub user_addr: String,
    pub country_geoname_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_geoname_id: Option<i64>,
    pub rate_per_kb: i64,
    pub rate_per_second: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prioritized_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prioritized_ip_level: Option<PrioritizedIPLevel>,
    pub created_at: i64,
    // ở đây sẽ ko có updated at, vi sẽ sử dụng nó cho việc quyết định khi nào sync data khi request nhie
    // bây giờ cứ update luôn cũng ko sợ
    // nên se luôn băng created_at
    pub updated_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_timeout: Option<i64>, // Timeout for the session
    // Các field cho algo proxy integration
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub username: String, // Username của proxy account
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub proxy_type: String, // algo, Enterprise, MMO, Dedicated
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub proxy_ip: String, // IP của proxy server/masternode
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub proxy_port: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_public_ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_ipu32: Option<u32>,
    // Các field cho status và ban management
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub status: String, // active, inactive, suspended, pending
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub ban_status: String, // none, warning, temporary, permanent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banned_until: Option<String>, // Thời gian hết hạn ban (ISO string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ban_reason: Option<String>, // Lý do ban/suspend
}

impl ProxyAccData {
    pub fn new(
        username: String,
        password: String,
        proxy_type: String,
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
        session_timeout: Option<i64>,
    ) -> Self {
        let mut _self = Self {
            id: username.clone(),
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
            updated_at: created_at,
            session_timeout: session_timeout.map(|t| t.clamp(30, 180)), // Clamp between 30-180 seconds
            // Các field mới - set default values
            username: username, // Default empty string
            proxy_type: proxy_type,
            proxy_ip: "".to_string(), // Default empty string
            proxy_port: "".to_string(), // Default empty string
            provider_public_ip_address: None,
            ttl: None,
            peer_ipu32: None,
            status: ProxyStatus::Active.to_string(), // Default to active
            ban_status: BanStatus::None.to_string(), // Default to none
            banned_until: None,
            ban_reason: None,
        };

        // Sử dụng username làm ID trực tiếp thay vì hash
        // user_4u28Lt1EuWqo nó là quá đủ cho 18 tỉ user tỉ lệ trùng rất thấp
        // let proto: ProtoProxyAcc = _self.clone().into();
        // let binding = ::prost::Message::encode_to_vec(&proto);
        // let bz: &[u8] = binding.as_slice();
        // _self.id = bytes_to_hex_string(hash(bz).as_bytes());
        _self
    }

    /// Get session timeout with validation (30s min, 180s max, default 180s)
    pub fn get_session_timeout(&self) -> i64 {
        self.session_timeout
            .map(|t| t.clamp(30, 180))
            .unwrap_or(180)
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
