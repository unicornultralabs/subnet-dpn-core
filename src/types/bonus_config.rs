use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
pub struct BonusConfig {
    country_geoname_id: i32,
    bonus_amount: f64,
}

impl BonusConfig {
    pub fn new(country_geoname_id: i32, bonus_amount: f64) -> Self {
        Self {
            country_geoname_id,
            bonus_amount,
        }
    }
}
