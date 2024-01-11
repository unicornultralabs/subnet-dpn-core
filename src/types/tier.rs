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
    Silver,
    Gold,
    Platinum,
    Diamond,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TierPoint {
    pub user_addr: String,
    pub points: i16,
    pub points_type: PointType,
}

#[derive(Debug, Clone, FromPrimitive, Serialize, Deserialize, ToSchema)]
pub enum PointType {
    Network,
    Task,
}
