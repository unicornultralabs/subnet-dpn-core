use chrono::Utc;
use dpn_proto::internal_tx::ProtoInternalTx;
use ethers::types::H256;
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use web3::types::{Address, U256};

use crate::utils::{bytes_to_hex_string, hash::hash, u256_to_szabo};

use super::tx::TxStatus;

#[derive(Debug, Clone, Serialize, Deserialize, FromPrimitive, ToSchema)]
pub enum InternalTxType {
    Network,
    Task,
    Referral,
    /// deprecated: Comission now is PlatformFee or ReferralFee
    Commission,
    PlatformFee,
    ReferralFee,
    ReferralFeeV2,
    Transfer,
    Quest,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InternalTx {
    pub tx_hash: H256,
    pub from_addr: Address,
    pub to_addr: Address,
    pub amount: U256,
    pub tx_type: InternalTxType,
    pub tx_status: TxStatus,
    pub created_at: i64,
}

impl InternalTx {
    pub fn new(
        from_addr: Address,
        to_addr: Address,
        amount: U256,
        tx_type: InternalTxType,
        tx_status: TxStatus,
    ) -> Self {
        let created_at_micros = Utc::now().timestamp_micros();

        let mut _self = Self {
            tx_hash: H256::zero(),
            from_addr,
            to_addr,
            amount,
            tx_type,
            tx_status,
            created_at: created_at_micros,
        };

        let proto: ProtoInternalTx = _self.clone().into();
        let binding = ::prost::Message::encode_to_vec(&proto);
        let bz = binding.as_slice();
        let tx_hash = hash(bz);
        _self.tx_hash = tx_hash;

        // TODO(rameight): we use microsecs to avoid hash collision
        // now we convert microsecs to secs back
        _self.created_at /= 1_000_000;
        _self
    }
}

impl Into<ProtoInternalTx> for InternalTx {
    fn into(self) -> ProtoInternalTx {
        ProtoInternalTx {
            from_addr: bytes_to_hex_string(self.from_addr.as_bytes()),
            to_addr: bytes_to_hex_string(self.to_addr.as_bytes()),
            amount: u256_to_szabo(self.amount.clone()),
            tx_status: self.tx_status as i32,
            tx_type: self.tx_type as i32,
            created_at: self.created_at,
        }
    }
}
