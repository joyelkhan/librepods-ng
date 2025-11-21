# ADR 0001: Bluetooth Abstraction Layer

## Status
Accepted

## Context
LibrePods NG must support 4 platforms: Linux, Windows, macOS, and Android. Each platform has a different Bluetooth API:
- Linux: BlueZ D-Bus
- Windows: WinRT API
- macOS: CoreBluetooth framework
- Android: Android Bluetooth API via JNI

## Decision
Implement a trait-based abstraction layer with platform-specific implementations.

## Rationale
- Trait-based design allows compile-time platform selection
- Each backend is isolated and testable
- No runtime overhead (monomorphization)
- Easy to add new platforms

## Consequences
- Positive: Clean separation of concerns, platform-agnostic core logic
- Negative: Must maintain 4 separate implementations
- Mitigation: Comprehensive test suite for each backend

## Implementation
```rust
pub trait BluetoothBackend {
    fn start_scan(&mut self) -> Result<()>;
    fn connect(&mut self, address: &str) -> Result<()>;
    fn write_characteristic(&mut self, address: &str, uuid: &str, data: &[u8]) -> Result<()>;
}
```
