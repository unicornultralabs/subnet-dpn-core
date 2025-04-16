use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RewardsOverview {
    /// claimed + unclaimed
    pub total_rewards: i64,
    /// unclaimed
    pub unclaimed_rewards: i64,
    /// claimed + unclaimed
    pub total_network_rewards: i64,
    /// claimed + unclaimed
    pub total_task_rewards: i64,
    /// claimed + unclaimed
    pub total_referral_rewards: i64,
    /// claimed + unclaimed
    pub total_commission_rewards: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RewardsOverviewV2 {
    /// claimed + unclaimed
    pub total_rewards: i64,
    /// claimed + unclaimed u2u rewards
    pub total_rewards_v2: i64,
    /// unclaimed
    pub unclaimed_rewards: i64,
    /// unclaimed u2u rewards
    pub unclaimed_rewards_v2: i64,
    /// claimed + unclaimed
    pub total_network_rewards: i64,
    pub total_network_rewards_v2: i64,
    /// claimed + unclaimed
    pub total_task_rewards: i64,
    /// claimed + unclaimed
    pub total_referral_rewards: i64,
    pub total_referral_rewards_v2: i64,
    /// claimed + unclaimed
    pub total_commission_rewards: i64,
}


