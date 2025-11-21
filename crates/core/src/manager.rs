//! High-level device manager

use crate::*;
use std::collections::HashMap;

/// Device manager for coordinating multiple AirPods devices
pub struct DeviceManager {
    devices: HashMap<String, Device>,
    event_bus: EventBus,
}

impl DeviceManager {
    /// Create new device manager
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
            event_bus: EventBus::new(),
        }
    }

    /// Add device
    pub fn add_device(&mut self, device: Device) -> Result<()> {
        self.devices.insert(device.id().to_string(), device);
        Ok(())
    }

    /// Remove device
    pub fn remove_device(&mut self, id: &str) -> Result<Option<Device>> {
        Ok(self.devices.remove(id))
    }

    /// Get device
    pub fn get_device(&self, id: &str) -> Option<&Device> {
        self.devices.get(id)
    }

    /// Get mutable device
    pub fn get_device_mut(&mut self, id: &str) -> Option<&mut Device> {
        self.devices.get_mut(id)
    }

    /// List all devices
    pub fn list_devices(&self) -> Vec<&Device> {
        self.devices.values().collect()
    }

    /// Get connected devices
    pub fn connected_devices(&self) -> Vec<&Device> {
        self.devices
            .values()
            .filter(|d| *d.state() == DeviceState::Connected)
            .collect()
    }

    /// Update device state
    pub fn update_device_state(&mut self, id: &str, state: DeviceState) -> Result<()> {
        if let Some(device) = self.devices.get_mut(id) {
            device.set_state(state);
            Ok(())
        } else {
            Err(Error::DeviceNotConnected)
        }
    }
}

impl Default for DeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = DeviceManager::new();
        assert_eq!(manager.list_devices().len(), 0);
    }

    #[test]
    fn test_add_device() {
        let mut manager = DeviceManager::new();
        let device = Device::new(
            "test_001".to_string(),
            "Test AirPods".to_string(),
            DeviceModel::AirPods3,
        );
        manager.add_device(device).unwrap();
        assert_eq!(manager.list_devices().len(), 1);
    }
}
