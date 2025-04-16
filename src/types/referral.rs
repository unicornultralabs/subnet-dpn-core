use ethers::types::H256;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use web3::types::Address;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Referral {
    pub user_addr: Address,
    pub referral_code: Option<String>,
    pub created_at: i64,
    pub referred_by: Option<Address>,
    pub referred_at: Option<i64>,
    pub tx_hash: Option<H256>,
}

impl Referral {
    pub fn new(
        user_addr: Address,
        referral_code: Option<String>,
        created_at: i64,
        referred_by: Option<Address>,
        referred_at: Option<i64>,
        tx_hash: Option<H256>,
    ) -> Self {
        Self {
            referral_code: referral_code,
            user_addr: user_addr,
            created_at: created_at,
            referred_by: referred_by,
            referred_at: referred_at,
            tx_hash: tx_hash,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReferralsOverview {
    pub total_referees: i64,
    // number of commission transactions from referees
    pub total_commission_txs: i64,
    // commision earned from referees
    pub total_commision: i64,
    // unclaimed commision earned from referees
    pub unclaimed_commission: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReferralsOverviewV2 {
    pub total_referees: i64,
    // number of commission transactions from referees
    pub total_commission_txs: i64,
    // commision earned from referees
    pub total_commision: i64,
    // unclaimed commision earned from referees
    pub unclaimed_commission: i64,
    // commision earned from referees
    pub total_commision_v2: i64,
    // unclaimed commision earned from referees
    pub unclaimed_commission_v2: i64,
}