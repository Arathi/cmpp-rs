use actix_codec::{Decoder, Encoder};
use bytes::{Buf, BufMut, BytesMut};

pub const CMPP_CONNECT: u32 = 0x0000_0001;
pub const CMPP_CONNECT_RESP: u32 = 0x8000_0001;
pub const CMPP_TERMINATE: u32 = 0x0000_0002;
pub const CMPP_TERMINATE_RESP: u32 = 0x8000_0002;
pub const CMPP_SUBMIT: u32 = 0x0000_0004;
pub const CMPP_SUBMIT_RESP: u32 = 0x8000_0004;
pub const CMPP_DELIVER: u32 = 0x0000_0005;
pub const CMPP_DELIVER_RESP: u32 = 0x8000_0005;
pub const CMPP_ACTIVE_TEST: u32 = 0x0000_0008;
pub const CMPP_ACTIVE_TEST_RESP: u32 = 0x8000_0008;

pub struct Header {
    total_length: u32,
    command_id: u32,
    sequence_id: u32,
}

#[derive(Default)]
pub struct HeaderCodec;

impl Decoder for HeaderCodec {
    type Item = Header;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 12 {
            return Ok(None);
        }

        let total_length = src.get_u32();
        let command_id = src.get_u32();
        let sequence_id = src.get_u32();
        let header = Header {
            total_length,
            command_id,
            sequence_id,
        };
        Ok(Some(header))
    }
}

impl Encoder<Header> for HeaderCodec {
    type Error = std::io::Error;

    fn encode(&mut self, header: Header, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_u32(header.total_length);
        dst.put_u32(header.command_id);
        dst.put_u32(header.sequence_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER_LENGTH: u32 = 12;
    const COMMAND_ID: u32 = CMPP_ACTIVE_TEST;
    const SEQUENCE_ID: u32 = 0x12345678;

    #[test]
    fn test_decode() {
        let mut codec = HeaderCodec::default();
        let mut bytes = BytesMut::new();
        
        bytes.put_u32(HEADER_LENGTH);
        bytes.put_u32(COMMAND_ID);
        bytes.put_u32(SEQUENCE_ID);

        let result = codec.decode(&mut bytes);
        match result {
            Ok(item) => {
                if let Some(header) = item {
                    println!("total_length = {}", header.total_length);
                    assert_eq!(HEADER_LENGTH, header.total_length);

                    println!("command_id = 0x{:08x}", header.command_id);
                    assert_eq!(COMMAND_ID, header.command_id);
                    
                    println!("sequence_id = {}", header.sequence_id);
                    assert_eq!(SEQUENCE_ID, header.sequence_id);
                }
            }
            Err(error) => {
                println!("Header解码出现异常：{:?}", error);
            }
        }
    }

    #[test]
    fn test_encode() {
        let mut codec = HeaderCodec::default();
        let header = Header {
            total_length: HEADER_LENGTH,
            command_id: COMMAND_ID,
            sequence_id: SEQUENCE_ID,
        };
        let mut bytes = BytesMut::new();
        let result = codec.encode(header, &mut bytes);

        match result {
            Ok(()) => {
                let total_length = bytes.get_u32();
                let command_id = bytes.get_u32();
                let sequence_id = bytes.get_u32();
        
                println!("total_length = {}", total_length);
                assert_eq!(HEADER_LENGTH, total_length);
        
                println!("command_id = 0x{:08x}", command_id);
                assert_eq!(COMMAND_ID, command_id);
        
                println!("sequence_id = {}", sequence_id);
                assert_eq!(SEQUENCE_ID, sequence_id);
            }
            Err(error) => {
                println!("Header编码出现异常：{:?}", error);
            }
        }
        
    }
}

