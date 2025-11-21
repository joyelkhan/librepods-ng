//! Data models for AirPods features

use serde::{Deserialize, Serialize};

/// ANC (Active Noise Cancellation) modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AncMode {
    Off = 0,
    Active = 1,
    Transparency = 2,
    Adaptive = 3,
}

/// Ear detection state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EarDetectionState {
    Unknown,
    LeftEarIn,
    RightEarIn,
    BothEarsIn,
    BothEarsOut,
}

/// Conversation awareness state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConversationAwarenessState {
    Inactive,
    Active,
    Speaking,
}

/// Spatial audio configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialAudioConfig {
    pub enabled: bool,
    pub head_tracking: bool,
    pub dynamic_head_tracking: bool,
}

/// Hearing aid mode configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HearingAidConfig {
    pub enabled: bool,
    pub amplification_level: u8,
    pub frequency_response: Vec<u8>,
}

/// Device rename request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameRequest {
    pub device_id: String,
    pub new_name: String,
}

/// Multipoint connection info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipointInfo {
    pub enabled: bool,
    pub connected_devices: Vec<String>,
    pub active_device: Option<String>,
}

/// Heart rate measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartRateMeasurement {
    pub bpm: u16,
    pub confidence: u8,
    pub timestamp: u64,
}

/// FindMy location data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindMyLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f32,
    pub timestamp: u64,
}

/// Long press action configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LongPressAction {
    Siri,
    PlayPause,
    NextTrack,
    PreviousTrack,
    VolumeUp,
    VolumeDown,
}

/// Custom transparency configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTransparencyConfig {
    pub enabled: bool,
    pub ambient_mix_level: u8,
    pub voice_focus: bool,
}

/// Head gesture configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadGestureConfig {
    pub double_tap_enabled: bool,
    pub double_tap_action: Option<LongPressAction>,
}
