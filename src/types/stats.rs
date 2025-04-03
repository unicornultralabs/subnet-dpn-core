use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ProviderByCountryStats {
    iso_code: String,
    name: String,
    geoname_id: String,
    total_active_ip: u32,
    total_active_user: u32,
    total_active_bandwidth: u64,
    
}