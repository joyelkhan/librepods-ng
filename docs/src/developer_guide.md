# LibrePods NG Developer Guide

## Architecture

### Core Engine (Rust)
- **Bluetooth Abstraction**: Platform-specific backends (BlueZ, WinRT, CoreBluetooth, Android JNI)
- **Protocol Parser**: AAP protocol implementation with 15 message types
- **State Machine**: Per-device state management
- **Crypto**: AES-256-GCM encryption with HKDF key derivation

### FFI Bridge
- Rust ↔ Dart/Flutter communication
- Memory-safe C ABI
- Async message passing

### Flutter UI
- Cross-platform (Linux, Windows, macOS, Android, iOS)
- Material Design 3
- Localized in 6 languages

## Building

```bash
just build-all
```

## Testing

```bash
just test-all
```

## Code Structure

```
crates/core/src/
├── backends/          # Platform-specific Bluetooth
├── protocol.rs        # AAP message types
├── crypto.rs          # Encryption/decryption
├── security.rs        # Replay window, constant-time ops
├── device.rs          # Device state
└── ...
```

## Adding a New Message Type

1. Add variant to `MessageType` enum in `protocol.rs`
2. Implement serialization/deserialization
3. Add handler in `Device::handle_message()`
4. Write tests in `tests/protocol_test.rs`

## Bluetooth Backend Implementation

Each backend must implement:
- `start_scan()` - Begin device discovery
- `stop_scan()` - End device discovery
- `connect(address)` - Establish connection
- `disconnect(address)` - Close connection
- `write_characteristic()` - Send data
- `read_characteristic()` - Receive data

## Security Considerations

- All keys are zeroized on drop
- Replay attacks prevented with nonce window
- Constant-time comparison for authentication
- No unsafe code (`#![forbid(unsafe_code)]`)

## Release Process

1. Update version in `Cargo.toml`
2. Create git tag: `git tag v1.0.0`
3. Push tag: `git push origin v1.0.0`
4. GitHub Actions builds and releases artifacts

## Contributing

1. Fork repository
2. Create feature branch
3. Implement changes
4. Run `cargo test --all-features`
5. Submit pull request

## License

GNU General Public License v3.0 with library exception
