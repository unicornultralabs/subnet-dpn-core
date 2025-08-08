use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserClaims {
    pub user_addr: String,
    pub login_session_id: String,
    pub exp: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(serde::Deserialize, Debug, Clone, ToSchema)]
pub enum SSOInfo {
    Google(GoogleSSOInfo),
    Apple(AppleSSOInfo),
}

#[derive(serde::Deserialize, Debug, Clone, ToSchema)]
pub struct GoogleSSOInfo {
    pub id: String,
    pub display_name: String,
    pub photo_url: String,
    pub token: String,
}

#[derive(serde::Deserialize, Debug, Clone, ToSchema)]
pub struct AppleSSOInfo {
    pub full_name: String,
    pub code: String,
    pub token: String,
}

#[derive(serde::Deserialize, Debug, Clone, ToSchema)]
pub struct UserSignInReq {
    pub username: String,
    pub password: String,
}

#[derive(serde::Deserialize, Debug, Clone, ToSchema)]
pub struct UserSignUpReq {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct UserSignUpResp {
    pub user_id: String,
}


#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct SSORes {
    pub code: i16,
    pub user_id: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub error_msg: String,
    pub is_first_sign_in: bool,
    pub reward: Option<RewardSigning>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardSigning {
    pub id: i32,
    pub amount: i32,
    pub reward_id : i32,
    pub probability: i32,
    pub band_name: String,
    pub stock: i32,
    pub status: i32,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}