use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

pub const AAP_SERVICE_UUID: u128 = 0x7DFC90007D1C495186AA8D9728F8D66C;
pub const AAP_CHARACTERISTIC_UUID: u128 = 0x7DFC90017D1C495186AA8D9728F8D66C;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum MessageType {
    BatteryStatus = 0x01,
    AncControl = 0x02,
    EarDetection = 0x03,
    FirmwareInfo = 0x04,
    SpatialAudio = 0x05,
    HeartRate = 0x06,
    FindMy = 0x07,
    ConversationAwareness = 0x08,
    HearingAid = 0x09,
    DeviceRename = 0x0A,
    MultipointControl = 0x0B,
    AdaptiveTransparency = 0x0C,
    LongPressActions = 0x0D,
    CustomTransparency = 0x0E,
    HeadGestures = 0x0F,
}

impl MessageType {
    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            0x01 => Ok(MessageType::BatteryStatus),
            0x02 => Ok(MessageType::AncControl),
            0x03 => Ok(MessageType::EarDetection),
            0x04 => Ok(MessageType::FirmwareInfo),
            0x05 => Ok(MessageType::SpatialAudio),
            0x06 => Ok(MessageType::HeartRate),
            0x07 => Ok(MessageType::FindMy),
            0x08 => Ok(MessageType::ConversationAwareness),
            0x09 => Ok(MessageType::HearingAid),
            0x0A => Ok(MessageType::DeviceRename),
            0x0B => Ok(MessageType::MultipointControl),
            0x0C => Ok(MessageType::AdaptiveTransparency),
            0x0D => Ok(MessageType::LongPressActions),
            0x0E => Ok(MessageType::CustomTransparency),
            0x0F => Ok(MessageType::HeadGestures),
            _ => Err(Error::UnknownMessageType(value)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub msg_type: MessageType,
    pub payload: Vec<u8>,
    pub crc: u16,
}

impl Message {
    pub fn new(msg_type: MessageType, payload: Vec<u8>) -> Self {
        let crc = Self::calculate_crc(&payload);
        Self {
            msg_type,
            payload,
            crc,
        }
    }

    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len() < 3 {
            return Err(Error::InvalidLength);
        }

        let msg_type = MessageType::from_u8(data[0])?;
        let len = data[1] as usize;

        if data.len() < 3 + len {
            return Err(Error::InvalidLength);
        }

        let payload = data[2..2 + len].to_vec();
        let crc = u16::from_be_bytes([data[2 + len], data[3 + len]]);

        let calculated_crc = Self::calculate_crc(&payload);
        if calculated_crc != crc {
            return Err(Error::CrcMismatch);
        }

        Ok(Self {
            msg_type,
            payload,
            crc,
        })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result = vec![self.msg_type as u8, self.payload.len() as u8];
        result.extend_from_slice(&self.payload);
        result.extend_from_slice(&self.crc.to_be_bytes());
        result
    }

    fn calculate_crc(data: &[u8]) -> u16 {
        let crc = crc::Crc::<u16>::new(&crc::CRC_16_IBM_SDLC);
        crc.checksum(data)
    }
}
