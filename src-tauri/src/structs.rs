use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct XZMUAccount {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XZMUNetConfig {
    wlan_user_ip: String,
    wlan_user_mac: String,
    wlan_ac_ip: String,
    wlan_ac_name: String,
}
