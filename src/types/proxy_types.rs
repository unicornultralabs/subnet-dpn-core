use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Enum representing proxy types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProxyType {
    Default,
    Enterprise, // Partner A - 1:1 mapping
    #[serde(rename = "MMO")]
    MMO,        // Partner B - region rotation
    Dedicated,
    #[serde(rename = "algo")]
    Algo,       // Dynamic algo proxy
}

impl fmt::Display for ProxyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProxyType::Default => write!(f, "Default"),
            ProxyType::Enterprise => write!(f, "Enterprise"),
            ProxyType::MMO => write!(f, "MMO"),
            ProxyType::Dedicated => write!(f, "Dedicated"),
            ProxyType::Algo => write!(f, "algo"),
        }
    }
}

impl FromStr for ProxyType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Default" => Ok(ProxyType::Default),
            "Enterprise" => Ok(ProxyType::Enterprise),
            "MMO" => Ok(ProxyType::MMO),
            "Dedicated" => Ok(ProxyType::Dedicated),
            "algo" => Ok(ProxyType::Algo),
            _ => Err(format!("Invalid proxy type: {}", s)),
        }
    }
}

impl From<ProxyType> for String {
    fn from(proxy_type: ProxyType) -> Self {
        proxy_type.to_string()
    }
}

impl TryFrom<String> for ProxyType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

/// Status enum for proxy accounts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProxyStatus {
    Active,
    Inactive,
    Suspended,
    Pending,
}

impl fmt::Display for ProxyStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProxyStatus::Active => write!(f, "active"),
            ProxyStatus::Inactive => write!(f, "inactive"),
            ProxyStatus::Suspended => write!(f, "suspended"),
            ProxyStatus::Pending => write!(f, "pending"),
        }
    }
}

impl FromStr for ProxyStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(ProxyStatus::Active),
            "inactive" => Ok(ProxyStatus::Inactive),
            "suspended" => Ok(ProxyStatus::Suspended),
            "pending" => Ok(ProxyStatus::Pending),
            _ => Err(format!("Invalid proxy status: {}", s)),
        }
    }
}

/// Ban status enum for proxy accounts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BanStatus {
    None,
    Warning,
    Temporary,
    Permanent,
}

impl fmt::Display for BanStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BanStatus::None => write!(f, "none"),
            BanStatus::Warning => write!(f, "warning"),
            BanStatus::Temporary => write!(f, "temporary"),
            BanStatus::Permanent => write!(f, "permanent"),
        }
    }
}

impl FromStr for BanStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(BanStatus::None),
            "warning" => Ok(BanStatus::Warning),
            "temporary" => Ok(BanStatus::Temporary),
            "permanent" => Ok(BanStatus::Permanent),
            _ => Err(format!("Invalid ban status: {}", s)),
        }
    }
}
