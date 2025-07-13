use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    SendSms,
    ViewSms,
}

impl FromStr for Permission {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sms.send" => Ok(Permission::SendSms),
            "sms.view" => Ok(Permission::ViewSms),
            _ => Err(anyhow!("Invalid permission: {s}")),
        }
    }
}
