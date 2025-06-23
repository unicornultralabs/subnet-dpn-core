
use dpn_proto::user_balance::{ProtoBalanceChange, ProtoRefreshBalances, ProtoUserBalance};
use prost::Message;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserTask {
    pub user_addr: String,
}
