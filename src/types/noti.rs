use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct NotificationRegister {
    pub user_addr: String,
    pub email: String,
    pub token: String,
    pub device_type: String,
}
