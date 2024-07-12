use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct UserBalance {
    pub balance: i64,
}