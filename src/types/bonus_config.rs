use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
pub struct BonusConfig {
    pub country_geoname_id: i32,
    pub country_name: String,
    pub bonus_amount: f64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl BonusConfig {
    pub fn new(
        country_geoname_id: i32,
        country_name: String,
        bonus_amount: f64,
        created_at: i64,
        updated_at: i64,
        ) -> Self {
        Self {
            country_geoname_id,
            country_name,
            bonus_amount,
            created_at,
            updated_at,
        }
    }
}
