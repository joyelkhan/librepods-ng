//! Bluetooth backend implementations

#[cfg(target_os = "linux")]
pub mod bluez;

#[cfg(target_os = "macos")]
pub mod corebluetooth;

#[cfg(target_os = "windows")]
pub mod winrt;

#[cfg(target_os = "android")]
pub mod android;
