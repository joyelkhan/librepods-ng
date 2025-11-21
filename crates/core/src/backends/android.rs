//! Android JNI backend

#[cfg(target_os = "android")]
use crate::bluetooth::BluetoothBackend;
use crate::error::Result;

#[cfg(target_os = "android")]
pub struct AndroidBackend;

#[cfg(target_os = "android")]
impl BluetoothBackend for AndroidBackend {
    fn start_scan(&mut self) -> Result<()> {
        Ok(())
    }

    fn stop_scan(&mut self) -> Result<()> {
        Ok(())
    }

    fn connect(&mut self, _address: &str) -> Result<()> {
        Ok(())
    }

    fn disconnect(&mut self, _address: &str) -> Result<()> {
        Ok(())
    }

    fn write_characteristic(
        &mut self,
        _address: &str,
        _service_uuid: u128,
        _char_uuid: u128,
        _data: &[u8],
    ) -> Result<()> {
        Ok(())
    }

    fn read_characteristic(
        &mut self,
        _address: &str,
        _service_uuid: u128,
        _char_uuid: u128,
    ) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }

    fn enable_notifications(
        &mut self,
        _address: &str,
        _service_uuid: u128,
        _char_uuid: u128,
    ) -> Result<()> {
        Ok(())
    }
}
