//! AAP Protocol message parser

use crate::error::Result;
use crate::protocol::{Message, MessageType};
use nom::{
    bytes::complete::take,
    number::complete::be_u8,
    IResult,
};

/// Parse AAP message from bytes
pub fn parse_message(input: &[u8]) -> IResult<&[u8], Message> {
    let (input, msg_type_byte) = be_u8(input)?;
    let msg_type = MessageType::from_u8(msg_type_byte)
        .map_err(|_| nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify)))?;
    
    let (input, length) = be_u8(input)?;
    let (input, payload) = take(length as usize)(input)?;
    let (input, crc_bytes) = take(2usize)(input)?;
    
    let crc = u16::from_be_bytes([crc_bytes[0], crc_bytes[1]]);
    
    Ok((input, Message {
        msg_type,
        payload: payload.to_vec(),
        crc,
    }))
}

/// Parse battery status message
pub fn parse_battery_status(payload: &[u8]) -> Result<(u8, u8, u8)> {
    if payload.len() < 3 {
        return Err(crate::error::Error::InvalidLength);
    }
    Ok((payload[0], payload[1], payload[2]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_battery_status() {
        let payload = vec![50, 60, 70];
        let (left, right, case) = parse_battery_status(&payload).unwrap();
        assert_eq!(left, 50);
        assert_eq!(right, 60);
        assert_eq!(case, 70);
    }
}
