use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    DeviceDiscovered,
    DeviceConnected,
    DeviceDisconnected,
    BatteryUpdated,
    StateChanged,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event_type: EventType,
    pub device_id: String,
    pub payload: Vec<u8>,
    pub timestamp: u64,
}

pub type EventListener = Arc<Mutex<dyn Fn(&Event) + Send + Sync>>;

pub struct EventBus {
    listeners: Vec<EventListener>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }

    pub fn subscribe(&mut self, listener: EventListener) {
        self.listeners.push(listener);
    }

    pub fn emit(&self, event: &Event) {
        for listener in &self.listeners {
            if let Ok(callback) = listener.lock() {
                callback(event);
            }
        }
    }

    pub fn clear(&mut self) {
        self.listeners.clear();
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
