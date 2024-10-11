use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XZMUAccount {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XZMUNetConfig {
    pub(crate) wlan_user_ip: String,
    pub(crate) wlan_user_mac: String,
    pub(crate) wlan_ac_ip: String,
    pub(crate) wlan_ac_name: String,
}
