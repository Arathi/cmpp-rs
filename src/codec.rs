mod decoder;
mod encoder;

use crate::pdu::command::{
    CMPP_ACTIVE_TEST, CMPP_ACTIVE_TEST_RESP, CMPP_CONNECT, CMPP_CONNECT_RESP, CMPP_DELIVER,
    CMPP_DELIVER_RESP, CMPP_SUBMIT, CMPP_SUBMIT_RESP, CMPP_TERMINATE, CMPP_TERMINATE_RESP,
};

use crate::pdu::{Packet, Payload};
use actix_codec::{Decoder, Encoder};
use bytes::{Buf, BufMut, BytesMut};
use decoder::{
    decode_connect_request, decode_connect_response, decode_deliver_request, decode_header,
    decode_message_result, decode_submit_request,
};
use encoder::{
    encode_connect_request, encode_connect_response, encode_deliver_request, encode_header,
    encode_message_result, encode_submit_request,
};

#[derive(Debug, Default)]
pub struct PacketCodec;

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
                return Err(std::io::Error::other(format!(
                    "无效的命令 {:08x}",
                    item.header.command_id,
                )));
            }
        }
        Ok(())
    }
}
