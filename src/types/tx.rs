use chrono::Utc;
use dpn_proto::tx::ProtoTx;
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use web3::types::{Address, H256, U256};

use crate::utils::{bytes_to_hex_string, hash::hash, u256_to_szabo};

#[derive(Debug, Clone, FromPrimitive, Serialize, Deserialize, ToSchema)]
pub enum TxType {
    Deposit,
    Withdrawal,
    WithdrawalV2,
}

#[derive(Debug, Clone, FromPrimitive, Serialize, Deserialize, ToSchema)]
pub enum TxStatus {
    Failed,
    Success,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Tx {
    pub tx_hash: H256,
    pub from_addr: Address,
    pub to_addr: Address,
    pub amount: U256,
    pub tx_type: TxType,
    pub tx_status: TxStatus,
    pub chain_tx_hash: Option<H256>,
    pub created_at: i64,
}

impl Tx {
    pub fn new(
        from_addr: Address,
        to_addr: Address,
        amount: U256,
        tx_type: TxType,
        tx_status: TxStatus,
        chain_tx_hash: Option<H256>,
    ) -> Self {
        let created_at_micros = Utc::now().timestamp_micros();

        let mut _self = Self {
            tx_hash: H256::zero(),
            from_addr,
            to_addr,
            amount,
            tx_type,
            tx_status,
            chain_tx_hash,
            created_at: created_at_micros,
        };

        let proto: ProtoTx = _self.clone().into();
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

impl Into<ProtoTx> for Tx {
    fn into(self) -> ProtoTx {
        ProtoTx {
            from_addr: bytes_to_hex_string(self.from_addr.as_bytes()),
            to_addr: bytes_to_hex_string(self.to_addr.as_bytes()),
            amount: u256_to_szabo(self.amount.clone()),
            tx_status: self.tx_status as i32,
            tx_type: self.tx_type as i32,
            created_at: self.created_at,
        }
    }
}
