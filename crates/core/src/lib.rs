#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all)]

//! LibrePods Core Engine - Production-grade Rust implementation

pub mod error;
pub mod protocol;
pub mod device;
pub mod state;
pub mod crypto;
pub mod bluetooth;
pub mod events;
pub mod models;
pub mod parser;
pub mod backends;
pub mod upstream;
pub mod ingestion;
pub mod protocol_analyzer;
pub mod firmware_analyzer;
pub mod legal_scan;
pub mod dmca_scanner;
pub mod gpl_checker;
pub mod trademark_checker;
pub mod firmware_security;
pub mod protocol_drift;
pub mod protocol_comparator;
pub mod firmware_version_analyzer;
pub mod codebase_diff;
pub mod merge_planner;
pub mod three_way_diff;
pub mod implementation_executor;
pub mod change_applier;
pub mod verification;
pub mod ci_pipeline;
pub mod test_runner;
pub mod coverage_analyzer;
pub mod release_manager;
pub mod sbom_generator;
pub mod release_notes;

pub use error::{Error, Result};
pub use device::{Device, DeviceModel, DeviceCapability};
pub use protocol::{Message, MessageType};
pub use state::DeviceState;
pub use events::{Event, EventBus};
pub use models::*;

/// Core engine for managing AirPods devices
pub struct Engine {
    devices: std::collections::HashMap<String, Device>,
    event_bus: EventBus,
}

impl Engine {
    /// Create a new engine instance
    pub fn new() -> Self {
        Self {
            devices: std::collections::HashMap::new(),
            event_bus: EventBus::new(),
        }
    }

    /// Get mutable reference to event bus
    pub fn event_bus_mut(&mut self) -> &mut EventBus {
        &mut self.event_bus
    }

    /// Get reference to event bus
    pub fn event_bus(&self) -> &EventBus {
        &self.event_bus
    }

    /// Register a device
    pub fn register_device(&mut self, device: Device) {
        self.devices.insert(device.id().to_string(), device);
    }

    /// Get device by ID
    pub fn get_device(&self, id: &str) -> Option<&Device> {
        self.devices.get(id)
    }

    /// Get mutable device reference
    pub fn get_device_mut(&mut self, id: &str) -> Option<&mut Device> {
        self.devices.get_mut(id)
    }

    /// List all devices
    pub fn devices(&self) -> impl Iterator<Item = &Device> {
        self.devices.values()
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = Engine::new();
        assert_eq!(engine.devices().count(), 0);
    }
}
