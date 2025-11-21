use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceState {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryInfo {
    pub left_bud: u8,
    pub right_bud: u8,
    pub case: u8,
    pub is_charging: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AncMode {
    Off,
    Active,
    Transparency,
    Adaptive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStateInfo {
    pub connection_state: DeviceState,
    pub battery: Option<BatteryInfo>,
    pub anc_mode: Option<AncMode>,
    pub firmware_version: Option<String>,
    pub last_updated: u64,
}

impl DeviceStateInfo {
    pub fn new() -> Self {
        Self {
            connection_state: DeviceState::Disconnected,
            battery: None,
            anc_mode: None,
            firmware_version: None,
            last_updated: 0,
        }
    }
}

impl Default for DeviceStateInfo {
    fn default() -> Self {
        Self::new()
    }
}
