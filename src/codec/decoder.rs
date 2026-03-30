use crate::pdu::{
    ConnectRequestPayload, ConnectResponsePayload, DeliverRequestPayload, Header,
    MessageResultPayload, SubmitRequestPayload,
};
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

pub fn decode_header(src: &mut BytesMut) -> Header {
    let total_length = src.get_u32();
    let command_id = src.get_u32();
    let sequence_id = src.get_u32();

    Header {
        total_length,
        command_id,
        sequence_id,
    }
}

pub fn decode_connect_request(src: &mut BytesMut) -> ConnectRequestPayload {
    let source_addr = decode_octet_string::<6>(src);
    let authenticator_source = decode_slice::<16>(src);
    let version = src.get_u8().into();
    let timestamp = src.get_u32().into();

    ConnectRequestPayload {
        source_addr,
        authenticator_source,
        version,
        timestamp,
    }
}

pub fn decode_connect_response(src: &mut BytesMut) -> ConnectResponsePayload {
    let status = src.get_u8();
    let authenticator_ismg = decode_slice::<16>(src);
    let version = src.get_u8().into();

    ConnectResponsePayload {
        status,
        authenticator_ismg,
        version,
    }
}

pub fn decode_submit_request(src: &mut BytesMut) -> SubmitRequestPayload {
    let msg_id = src.get_u64().into();
    let pk_total = src.get_u8();
    let pk_number = src.get_u8();
    let registered_delivery = src.get_u8();
    let msg_level = src.get_u8();
    let service_id = decode_octet_string::<10>(src);
    let fee_user_type = src.get_u8();
    let fee_terminal_id = decode_octet_string::<21>(src);
    let tp_pid = src.get_u8();
    let tp_udhi = src.get_u8();
    let msg_fmt = src.get_u8();
    let msg_src = decode_octet_string::<6>(src);
    let fee_type = decode_octet_string::<2>(src);
    let fee_code = decode_octet_string::<6>(src);
    let valid_time = decode_octet_string::<17>(src);
    let at_time = decode_octet_string::<17>(src);
    let src_id = decode_octet_string::<21>(src);
    let dest_usr_tl = src.get_u8();
    let mut dest_terminal_id = vec![];
    for _ in 0..dest_usr_tl {
        let msisdn = decode_octet_string::<21>(src);
        dest_terminal_id.push(msisdn);
    }
    let msg_length = src.get_u8();
    let msg_content: Vec<u8> = decode_vec_u8(src, msg_length as usize);
    let reserve: [u8; 8] = decode_slice::<8>(src);

    SubmitRequestPayload {
        msg_id,
        pk_total,
        pk_number,
        registered_delivery,
        msg_content,
        msg_level,
        service_id,
        fee_user_type,
        fee_terminal_id,
        tp_pid,
        tp_udhi,
        msg_fmt,
        msg_src,
        fee_type,
        fee_code,
        valid_time,
        at_time,
        src_id,
        dest_usr_tl,
        dest_terminal_id,
        msg_length,
        reserve,
    }
}

pub fn decode_message_result(src: &mut BytesMut) -> MessageResultPayload {
    let msg_id = src.get_u64().into();
    let result = src.get_u8();

    MessageResultPayload { msg_id, result }
}

pub fn decode_deliver_request(src: &mut BytesMut) -> DeliverRequestPayload {
    let msg_id = src.get_u64().into();
    let dest_id = decode_octet_string::<21>(src);
    let service_id = decode_octet_string::<10>(src);
    let tp_pid = src.get_u8();
    let tp_udhi = src.get_u8();
    let msg_fmt = src.get_u8();
    let src_terminal_id = decode_octet_string::<21>(src);
    let registered_delivery = src.get_u8();
    let msg_length = src.get_u8();
    let msg_content = decode_vec_u8(src, msg_length as usize);
    let reserved = decode_slice::<8>(src);

    DeliverRequestPayload {
        msg_id,
        dest_id,
        service_id,
        tp_pid,
        tp_udhi,
        msg_fmt,
        src_terminal_id,
        registered_delivery,
        msg_length,
        msg_content,
        reserved,
    }
}
