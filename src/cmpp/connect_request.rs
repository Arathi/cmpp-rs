use actix_codec::Decoder;
use actix_codec::Encoder;
use bytes::Buf;
use bytes::BufMut;
// use super::error::Error;
use std::io::Error as IOError;
use std::io::ErrorKind;

#[derive(Debug, Default)]
pub struct ConnectRequest {
    // 6
    pub source_addr: String,

    // 16
    pub authenticator_source: String,

    // 1
    pub version: u8,

    // 4
    pub timestamp: u32,
}

pub struct ConnectRequestCodec;

impl Decoder for ConnectRequestCodec {
    type Item = ConnectRequest;
    type Error = IOError;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let bytes = src.split_to(6);
        let source_addr = String::from_utf8(bytes.to_vec()).map_err(|e| {
            IOError::new(ErrorKind::Other, "")
        })?;

        let bytes = src.split_to(16);
        let authenticator_source = String::from_utf8(bytes.to_vec()).map_err(|e| {
            IOError::new(ErrorKind::Other, "")
        })?;
        
        let version = src.get_u8();
        let timestamp = src.get_u32();

        let request = ConnectRequest {
            source_addr,
            authenticator_source,
            version,
            timestamp,
        };
        Ok(Some(request))
    }
}

impl Encoder<ConnectRequest> for ConnectRequestCodec {
    type Error = IOError;

    fn encode(&mut self, item: ConnectRequest, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        let bytes = item.source_addr.as_bytes();
        if bytes.len() != 6 {
            let error = IOError::new(ErrorKind::Other, "source_addr长度不为6");
            return Err(error);
        }
        dst.put_slice(bytes);

        let bytes = item.authenticator_source.as_bytes();
        if bytes.len() != 16 {
            let error = IOError::new(ErrorKind::Other, "authenticator_source长度不为16");
            return Err(error);
        }
        dst.put_slice(bytes);

        dst.put_u8(item.version);
        dst.put_u32(item.timestamp);
    
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        //
    }

    #[test]
    fn test_encode() {
        //
    }
}
