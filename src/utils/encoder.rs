use bytes::{BufMut, BytesMut};

pub fn encode_octet_string<const L: usize>(dst: &mut BytesMut, src: &String) {
    let mut buf = [0u8; L];
    buf.copy_from_slice(src.as_bytes());
    dst.put_slice(&buf);
}
