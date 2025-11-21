use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BluetoothDevice {
    pub address: String,
    pub name: String,
    pub rssi: i32,
    pub is_connected: bool,
}

pub trait BluetoothBackend: Send + Sync {
    fn start_scan(&mut self) -> Result<()>;
    fn stop_scan(&mut self) -> Result<()>;
    fn connect(&mut self, address: &str) -> Result<()>;
    fn disconnect(&mut self, address: &str) -> Result<()>;
    fn write_characteristic(
        &mut self,
        address: &str,
        service_uuid: u128,
        char_uuid: u128,
        data: &[u8],
    ) -> Result<()>;
    fn read_characteristic(
        &mut self,
        address: &str,
        service_uuid: u128,
        char_uuid: u128,
    ) -> Result<Vec<u8>>;
    fn enable_notifications(
        &mut self,
        address: &str,
        service_uuid: u128,
        char_uuid: u128,
    ) -> Result<()>;
}

pub struct BluetoothManager;

impl BluetoothManager {
    pub fn new() -> Self {
        Self
    }
}

impl Default for BluetoothManager {
    fn default() -> Self {
        Self::new()
    }
}
