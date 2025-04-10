use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserTier {
    pub user_addr: String,
    pub tier: Tier,
    pub points: i64,
}

#[derive(Debug, Clone, FromPrimitive, Serialize, Deserialize, ToSchema)]
pub enum Tier {
    None,
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TierPoint {
    pub user_addr: String,
    pub points: i64,
    pub created_at: i64,
}
