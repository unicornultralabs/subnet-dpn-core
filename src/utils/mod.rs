pub mod hash;

use ethers::utils::{format_units, parse_units};
use hex::encode;
pub use web3::types::{
    Address, Bytes, Log, TransactionRequest, H128, H160, H2048, H256, U128, U256, U64,
};

/// deprecated: use address_to_string instead
pub fn bytes_to_hex_string(bz: &[u8]) -> String {
    format!("0x{}", String::from(encode(bz)))
}

pub fn address_to_string(addr: Address) -> String {
    format!("0x{}", String::from(encode(addr.as_bytes())))
}

pub fn string_to_address(addr: String) -> Address {
    addr.parse::<Address>()
        .expect(&format!("parse invalid address addr={}", addr))
}

pub fn u256_to_szabo(value: U256) -> i64 {
    format_units(value, "szabo")
        .unwrap()
        .parse::<f64>()
        .unwrap() as i64
}

pub fn szabo_to_u256(value: i64) -> U256 {
    U256::from(parse_units(value, "szabo").unwrap())
}

pub fn u256_to_ether(value: U256) -> f64 {
    format_units(value, "ether")
        .unwrap()
        .parse::<f64>()
        .unwrap()
}
