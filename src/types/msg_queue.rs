use serde::{Deserialize, Serialize};

use super::{
    bandwidth::{EphemeralSession, SessionTerminationReason},
    connection::PeernodeInfo,
    internal_tx::InternalTx,
    noti::NotificationRegister,
    tx::{Tx, TxStatus},
};
// exchanges
pub const EVENTS_EXCHANGE: &str = "dpn-events";
pub const STATS_EXCHANGE: &str = "dpn-stats";
pub const TXS_EXCHANGE: &str = "dpn-txs";
pub const WITHDRAWALS_EXCHANGE: &str = "dpn-withdrawals";
pub const BALANCES_EXCHANGE: &str = "dpn-balances";
pub const NOTIFICATION_EXCHANGE: &str = "dpn-notifications";
// queues
pub const CONNECTION_EVENTS_ADMIN_QUEUE: &str = "connection-events_admin";
pub const CONNECTION_EVENTS_EXPLORER_QUEUE: &str = "connection-events_explorer";
pub const EVENTS_ACCOUNTNG_QUEUE: &str = "events_accounting";
pub const SESSION_EVENTS_ADMIN_QUEUE: &str = "session-events_admin";
pub const SESSION_EVENTS_EXPLORER_QUEUE: &str = "session-events_explorer";
pub const SESSION_EVENTS_WEBSOCKET_QUEUE: &str = "session-events_websocket";
pub const STATS_WEBSOCKET_QUEUE: &str = "stats_websocket";
pub const TXS_ADMIN_QUEUE: &str = "txs_admin";
pub const TXS_EXPLORER_QUEUE: &str = "txs_explorer";
pub const TXS_ONCHAIN_QUEUE: &str = "txs_onchain";
pub const BALANCES_QUEUE: &str = "balances";
pub const TAPPOINT_EVENT_QUEUE: &str = "tappoint-events_admin";
pub const NOTIFICATION_REGISTER_QUEUE: &str = "notification-register_admin";
// routing keys
pub const DEPOSIT_ROUTING_KEY: &str = "deposit";
pub const WITHDRAWAL_ROUTING_KEY: &str = "withdrawal";
pub const CONNECTION_ROUTING_KEY: &str = "connection";
pub const REFERRAL_ROUTING_KEY: &str = "referral";
pub const SESSION_ROUTING_KEY: &str = "session";
pub const TXS_ROUTING_KEY: &str = "txs";
pub const TAPPOINT_EVENT_ROUTING_KEY: &str = "tappoint";
pub const NOTIFICATION_REGISTER_ROUTING_KEY: &str = "register";

#[derive(Debug, Clone, Serialize, Deserialize)]

pub enum DPNEvent {
    // connection
    PeerConnected(PeerConnectedExtra),
    PeerDisconnected(PeerDisconnectedExtra),

    // session
    SessionCreated(SessionCreatedExtra),
    SessionTerminated(SessionTerminatedExtra),

    // admin
    Deposit(DepositExtra),
    Withdrawal(WithdrawalExtra),
    Referral(ReferralExtra),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnchainWithdrawalRequest {
    pub from: String,
    pub to: String,
    pub amount: i64,
    pub tx_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedTx {
    pub tx_hash: String,
    pub status: TxStatus,
    pub chain_tx_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientBalanceUpdate {
    pub user_addr: String,
    pub balance: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerConnectedExtra {
    pub masternode_id: String,
    pub peer_addr: String,
    pub login_session_id: String,
    pub info: PeernodeInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerDisconnectedExtra {
    pub masternode_id: String,
    pub peer_addr: String,
    pub login_session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionCreatedExtra {
    pub masternode_id: String,
    pub session: EphemeralSession,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionTerminatedExtra {
    pub masternode_id: String,
    pub session: EphemeralSession,
    pub reason: SessionTerminationReason,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositExtra {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub tx_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalExtra {
    pub user_addr: String,
    pub withdrawal_addr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralExtra {
    pub referrer_addr: String,
    pub referee_addr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DPNTx {
    Tx(Tx),
    InternalTx(InternalTx),
}

#[derive(Debug, Clone, Serialize)]
pub enum NotificationEvent {
    Register(NotificationRegister),
    // NoLongerOnlinePeers(Vec<String>)
}
