use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum Error {
    #[error("Invalid message length")]
    InvalidLength,
    #[error("CRC checksum mismatch")]
    CrcMismatch,
    #[error("Unknown message type: {0}")]
    UnknownMessageType(u8),
    #[error("Cryptographic operation failed")]
    CryptoError,
    #[error("Bluetooth error: {0}")]
    BluetoothError(String),
    #[error("Unsupported device model")]
    UnsupportedDevice,
    #[error("Protocol version mismatch")]
    VersionMismatch,
    #[error("Invalid device state")]
    InvalidState,
    #[error("Device not connected")]
    DeviceNotConnected,
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Operation timeout")]
    Timeout,
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("IO error: {0}")]
    IoError(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err.to_string())
    }
}
