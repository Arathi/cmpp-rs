use crate::pdu::command::{
    CMPP_ACTIVE_TEST, CMPP_ACTIVE_TEST_RESP, CMPP_CONNECT, CMPP_CONNECT_RESP, CMPP_DELIVER,
    CMPP_DELIVER_RESP, CMPP_SUBMIT, CMPP_SUBMIT_RESP, CMPP_TERMINATE, CMPP_TERMINATE_RESP,
};

use crate::pdu::{
    ConnectRequestPayload, ConnectResponsePayload, DeliverRequestPayload, Header,
    MessageResultPayload, Packet, Payload, SubmitRequestPayload,
};
use crate::utils::decoder::{decode_octet_string, decode_slice, decode_vec_u8};
use crate::utils::encoder::encode_octet_string;
use actix_codec::{Decoder, Encoder};
use bytes::{Buf, BufMut, BytesMut};

#[derive(Debug, Default)]
pub struct PacketCodec;

// region: decoders
fn decode_header(src: &mut BytesMut) -> Header {
    let total_length = src.get_u32();
    let command_id = src.get_u32();
    let sequence_id = src.get_u32();

    Header {
        total_length,
        command_id,
        sequence_id,
    }
}

fn decode_connect_request(src: &mut BytesMut) -> ConnectRequestPayload {
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

fn decode_connect_response(src: &mut BytesMut) -> ConnectResponsePayload {
    let status = src.get_u8();
    let authenticator_ismg = decode_slice::<16>(src);
    let version = src.get_u8().into();

    ConnectResponsePayload {
        status,
        authenticator_ismg,
        version,
    }
}

fn decode_submit_request(src: &mut BytesMut) -> SubmitRequestPayload {
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
    let dest_terminal_id = decode_octet_string::<21>(src);
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

fn decode_message_result(src: &mut BytesMut) -> MessageResultPayload {
    let msg_id = src.get_u64().into();
    let result = src.get_u8();

    MessageResultPayload { msg_id, result }
}

fn decode_deliver_request(src: &mut BytesMut) -> DeliverRequestPayload {
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
// endregion

// region: encoders
fn encode_header(dst: &mut BytesMut, header: &Header) {
    dst.put_u32(header.total_length);
    dst.put_u32(header.command_id);
    dst.put_u32(header.sequence_id);
}

fn encode_connect_request(dst: &mut BytesMut, payload: &ConnectRequestPayload) {
    encode_octet_string::<6>(dst, &payload.source_addr);
    dst.put_slice(&payload.authenticator_source);
    dst.put_u8(payload.version.clone().into());
    dst.put_u32(payload.timestamp.clone().into());
}

fn encode_connect_response(dst: &mut BytesMut, payload: &ConnectResponsePayload) {
    dst.put_u8(payload.status);
    dst.put_slice(&payload.authenticator_ismg);
    dst.put_u8(payload.version.clone().into());
}

fn encode_submit_request(dst: &mut BytesMut, payload: &SubmitRequestPayload) {
    dst.put_u64(payload.msg_id.clone().into());
    todo!()
}

fn encode_message_result(dst: &mut BytesMut, payload: &MessageResultPayload) {
    dst.put_u64(payload.msg_id.clone().into());
    dst.put_u8(payload.result);
}

fn encode_deliver_request(dst: &mut BytesMut, payload: &DeliverRequestPayload) {
    dst.put_u64(payload.msg_id.clone().into());
    todo!()
}
// endregion

impl Decoder for PacketCodec {
    type Item = Packet;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.remaining() < 12 {
            println!("报文头长度不足：{}", src.remaining());
            return Ok(None);
        }
        let header = decode_header(src);

        let payload_length = (header.total_length - 12) as usize;
        if src.remaining() < payload_length {
            println!("报文体长度不足：{}", src.remaining());
            return Ok(None);
        }

        let mut payload: Payload = Payload::Empty;
        match header.command_id {
            CMPP_CONNECT => {
                let crp = decode_connect_request(src);
                payload = Payload::from(crp);
            }
            CMPP_CONNECT_RESP => {
                let crp = decode_connect_response(src);
                payload = Payload::from(crp);
            }
            CMPP_TERMINATE => (),
            CMPP_TERMINATE_RESP => (),
            CMPP_SUBMIT => {
                let srp = decode_submit_request(src);
                payload = Payload::from(srp);
            }
            CMPP_SUBMIT_RESP => {
                let mrp = decode_message_result(src);
                payload = Payload::from(mrp);
            }
            CMPP_DELIVER => {
                let drp = decode_deliver_request(src);
                payload = Payload::from(drp);
            }
            CMPP_DELIVER_RESP => {
                let mrp = decode_message_result(src);
                payload = Payload::from(mrp);
            }
            CMPP_ACTIVE_TEST => (),
            CMPP_ACTIVE_TEST_RESP => {
                let reserved = src.get_u8();
                payload = Payload::ActiveTestResponse(reserved);
            }
            _ => {
                println!("无效的命令 {:08x}", header.command_id);
                return Ok(None);
            }
        }

        Ok(Some(Packet { header, payload }))
    }
}

impl Encoder<Packet> for PacketCodec {
    type Error = std::io::Error;

    fn encode(&mut self, item: Packet, dst: &mut BytesMut) -> Result<(), Self::Error> {
        encode_header(dst, &item.header);

        match item.payload {
            Payload::ConnectRequest(payload) => {
                encode_connect_request(dst, &payload);
            }
            Payload::ConnectResponse(payload) => {
                encode_connect_response(dst, &payload);
            }
            Payload::SubmitRequest(payload) => {
                encode_submit_request(dst, &payload);
            }
            Payload::MessageResult(payload) => {
                encode_message_result(dst, &payload);
            }
            Payload::DeliverRequest(payload) => {
                encode_deliver_request(dst, &payload);
            }
            Payload::ActiveTestResponse(payload) => {
                dst.put_u8(payload);
            }
            Payload::Empty => (),
            _ => {
                return Err(std::io::Error::other("无效的命令类型"));
            }
        }
        Ok(())
    }
}
