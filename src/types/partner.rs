use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct PartnerConfigCondition {
    pub throughput_from: Option<f64>,
    pub throughput_to: Option<f64>,

    pub packet_loss_from: Option<f64>,
    pub packet_loss_to: Option<f64>,

    pub jitter_from: Option<u128>,
    pub jitter_to: Option<u128>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct PartnerConfig {
    pub id: String,
    pub partner_name: String,
    pub conditions: PartnerConfigCondition,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema, sqlx::FromRow)]
pub struct NewPartnerConfig {
    pub partner_name: String,
    pub requirement: PartnerConfigCondition,
}

impl PartnerConfig {
    pub fn new(id: String, partner_name: String, requirement: PartnerConfigCondition) -> Self {
        Self {
            id,
            partner_name,
            conditions: requirement,
        }
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct PartnerConfigQuery {
    pub throughput_from: Option<f64>,
    pub throughput_to: Option<f64>,

    pub packet_loss_from: Option<f64>,
    pub packet_loss_to: Option<f64>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    pub jitter_from: Option<u128>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    pub jitter_to: Option<u128>,
}

#[derive(Deserialize, Clone)]
pub struct PartnerConfigPath {
    pub partner: String,
}

impl PartnerConfigQuery {
    pub fn to_query_string(&self) -> String {
        let mut params = vec![];

        if let Some(val) = self.throughput_from {
            params.push(format!("throughput_from={}", val));
        }
        if let Some(val) = self.throughput_to {
            params.push(format!("throughput_to={}", val));
        }
        if let Some(val) = self.packet_loss_from {
            params.push(format!("packet_loss_from={}", val));
        }
        if let Some(val) = self.packet_loss_to {
            params.push(format!("packet_loss_to={}", val));
        }
        if let Some(val) = self.jitter_from {
            params.push(format!("jitter_from={}", val));
        }
        if let Some(val) = self.jitter_to {
            params.push(format!("jitter_to={}", val));
        }

        if params.is_empty() {
            "".to_string()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}
