use serde::{Deserialize, Serialize};
use web3::types::{Address, U256};
use utoipa::ToSchema;


#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct User {
    pub email: Option<String>,
    pub username: Option<String>,
    pub email_verified: bool,
    pub fingerprint: Option<String>,
    pub pincode: Option<String>,
    pub deposit_addr: Address,
    pub withdrawal_addr: Option<Address>,
    pub salt:Option<String>,
    pub password_hashed:Option<String>,
    pub balance: U256,
    pub created_at: i64,
    pub last_login: i64,

}

impl User {
    pub fn new(
        email: Option<String>,
        username: Option<String>,
        email_verified: bool,
        fingerprint: Option<String>,
        pincode: Option<String>,
        deposit_addr: Address,
        withdrawal_addr: Option<Address>,
        salt:Option<String>,
        password_hashed:Option<String>,
        created_at: i64,
        last_login: i64,
    ) -> Self {
        Self {
            email,
            username,
            email_verified,
            fingerprint: fingerprint,
            pincode: pincode,
            deposit_addr: deposit_addr,
            withdrawal_addr: withdrawal_addr,
            salt: salt,
            password_hashed: password_hashed,
            balance: U256::zero(),
            created_at,
            last_login,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum UserType {
    DPN,
    VPN,
}

impl UserType {
    pub fn to_string(&self) -> String {
        match self {
            UserType::DPN => "DPN".to_string(),
            UserType::VPN => "VPN".to_string(),
        }
    }
}