use crate::pdu::{
    ConnectRequestPayload, ConnectResponsePayload, DeliverRequestPayload, Header,
    MessageResultPayload, SubmitRequestPayload,
};
use bytes::{BufMut, BytesMut};

pub fn encode_octet_string<const L: usize>(dst: &mut BytesMut, src: &String) {
    let mut buf = [0u8; L];
    let bytes = src.as_bytes();
    let len = bytes.len().min(L);
    buf.copy_from_slice(&bytes[..len]);
    dst.put_slice(&buf);
}

pub fn encode_header(dst: &mut BytesMut, header: &Header) {
    dst.put_u32(header.total_length);
    dst.put_u32(header.command_id);
    dst.put_u32(header.sequence_id);
}

pub fn encode_connect_request(dst: &mut BytesMut, payload: &ConnectRequestPayload) {
    encode_octet_string::<6>(dst, &payload.source_addr);
    dst.put_slice(&payload.authenticator_source);
    dst.put_u8(payload.version.clone().into());
    dst.put_u32(payload.timestamp.clone().into());
}

pub fn encode_connect_response(dst: &mut BytesMut, payload: &ConnectResponsePayload) {
    dst.put_u8(payload.status);
    dst.put_slice(&payload.authenticator_ismg);
    dst.put_u8(payload.version.clone().into());
}

pub fn encode_submit_request(dst: &mut BytesMut, payload: &SubmitRequestPayload) {
    dst.put_u64(payload.msg_id.clone().into());
    dst.put_u8(payload.pk_total);
    dst.put_u8(payload.pk_number);
    dst.put_u8(payload.registered_delivery);
    dst.put_u8(payload.msg_level);
    encode_octet_string::<10>(dst, &payload.service_id);
    dst.put_u8(payload.fee_user_type);
    encode_octet_string::<21>(dst, &payload.fee_terminal_id);
    dst.put_u8(payload.tp_pid);
    dst.put_u8(payload.tp_udhi);
    dst.put_u8(payload.msg_fmt);
    encode_octet_string::<6>(dst, &payload.msg_src);
    encode_octet_string::<2>(dst, &payload.fee_type);
    encode_octet_string::<6>(dst, &payload.fee_code);
    encode_octet_string::<17>(dst, &payload.valid_time);
    encode_octet_string::<17>(dst, &payload.at_time);
    encode_octet_string::<21>(dst, &payload.src_id);
    dst.put_u8(payload.dest_usr_tl);
    for i in 0..payload.dest_usr_tl {
        encode_octet_string::<21>(dst, &payload.dest_terminal_id[i as usize]);
    }
    dst.put_u8(payload.msg_length);
    dst.put_slice(&payload.msg_content);
    dst.put_slice(&payload.reserve);
}

pub fn encode_message_result(dst: &mut BytesMut, payload: &MessageResultPayload) {
    dst.put_u64(payload.msg_id.clone().into());
    dst.put_u8(payload.result);
}

pub fn encode_deliver_request(dst: &mut BytesMut, payload: &DeliverRequestPayload) {
    dst.put_u64(payload.msg_id.clone().into());
    encode_octet_string::<21>(dst, &payload.dest_id);
    encode_octet_string::<10>(dst, &payload.service_id);
    dst.put_u8(payload.tp_pid);
    dst.put_u8(payload.tp_udhi);
    dst.put_u8(payload.msg_fmt);
    encode_octet_string::<21>(dst, &payload.src_terminal_id);
    dst.put_u8(payload.registered_delivery);
    dst.put_u8(payload.msg_length);
    dst.put_slice(&payload.msg_content);
    dst.put_slice(&payload.reserved);
}
