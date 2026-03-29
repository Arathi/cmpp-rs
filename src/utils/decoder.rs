use bytes::{Buf, BytesMut};

pub fn decode_slice<const L: usize>(src: &mut BytesMut) -> [u8; L] {
    let mut buf = [0u8; L];
    src.copy_to_slice(&mut buf);
    buf
}

pub fn decode_octet_string<const L: usize>(src: &mut BytesMut) -> String {
    let mut buf = [0u8; L];
    src.copy_to_slice(&mut buf);
    String::from_utf8_lossy(&buf).to_string()
}

pub fn decode_vec_u8(src: &mut BytesMut, length: usize) -> Vec<u8> {
    let bytes = src.copy_to_bytes(length);
    bytes.to_vec()
}
