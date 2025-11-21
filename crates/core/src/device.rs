use crate::state::DeviceState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceModel {
    AirPods2,
    AirPods3,
    AirPods4,
    AirPodsProGen1,
    AirPodsProGen2,
    AirPodsProGen3,
    AirPodsMax,
    BeatsFitPro,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeviceCapability {
    BatteryMonitoring,
    NoiseControl,
    AdaptiveTransparency,
    EarDetection,
    ConversationAwareness,
    HeadGestures,
    HearingAid,
    CustomTransparency,
    DeviceRename,
    LongPressActions,
    Multipoint,
    FirmwareInfo,
    FindMy,
    HeartRate,
    SpatialAudio,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    id: String,
    name: String,
    model: DeviceModel,
    state: DeviceState,
    capabilities: Vec<DeviceCapability>,
    metadata: HashMap<String, String>,
}

impl Device {
    pub fn new(id: String, name: String, model: DeviceModel) -> Self {
        Self {
            id,
            name,
            model,
            state: DeviceState::Disconnected,
            capabilities: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn model(&self) -> DeviceModel {
        self.model
    }

    pub fn state(&self) -> &DeviceState {
        &self.state
    }

    pub fn set_state(&mut self, state: DeviceState) {
        self.state = state;
    }

    pub fn capabilities(&self) -> &[DeviceCapability] {
        &self.capabilities
    }

    pub fn add_capability(&mut self, capability: DeviceCapability) {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
        }
    }

    pub fn has_capability(&self, capability: DeviceCapability) -> bool {
        self.capabilities.contains(&capability)
    }

    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|s| s.as_str())
    }
}
