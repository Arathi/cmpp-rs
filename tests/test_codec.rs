use actix_codec::{Decoder, Encoder};
use actix_web::cookie::time;
use bytes::{Buf, BufMut, BytesMut};
use cmpp_lib::codec::*;
use cmpp_lib::pdu::command::*;
use cmpp_lib::pdu::*;
use cmpp_lib::utils::hex::format as format_hex;

// region: CMPP_CONNECT
#[test]
fn test_encode_connect_request() {
    const LENGTH: usize = (LENGTH_HEADER + LENGTH_CONNECT) as usize;
    let mut encoder = PacketCodec::default();

    let packet = Packet {
        header: Header {
            total_length: LENGTH as u32,
            command_id: CMPP_CONNECT.into(),
            sequence_id: 0x01234567,
        },
        payload: Payload::from(ConnectRequestPayload {
            source_addr: "935016".into(),
            authenticator_source: [0u8; 16],
            version: 0x20.into(),
            timestamp: Timestamp::now(),
        }),
    };

    let mut dst = BytesMut::with_capacity(LENGTH);
    let _ = encoder.encode(packet, &mut dst);
    // let mut bytes = [0u8; LENGTH];
    // dst.copy_to_slice(&mut bytes);
    let bytes = &dst[..LENGTH];
    println!("{}", format_hex(&bytes, " ", true));
}

#[test]
fn test_decode_connect_request() {
    const LENGTH: usize = (LENGTH_HEADER + LENGTH_CONNECT) as usize;
    let mut buf = BytesMut::with_capacity(LENGTH);
    buf.put_slice(&hex_literal::hex!(
        r"
        00 00 00 27 
        00 00 00 01 
        01 23 45 67 
        39 33 35 30 31 36 
        00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
        20 
        13 9D D8 BC
        "
    ));

    let mut decoder = PacketCodec::default();

    let result = decoder.decode(&mut buf);
    assert!(result.is_ok());

    let option = result.unwrap();
    assert!(option.is_some());

    let packet = option.unwrap();

    println!("报文长度：{0:} (0x{0:08x})", packet.header.total_length);
    assert_eq!(packet.header.total_length, 0x00_00_00_27);

    println!("命令类型：{0:} (0x{0:08x})", packet.header.command_id);
    assert_eq!(packet.header.command_id, CMPP_CONNECT);

    println!("消息流水：{0:} (0x{0:08x})", packet.header.sequence_id);
    assert_eq!(packet.header.sequence_id, 0x01_23_45_67);

    if let Payload::ConnectRequest(payload) = packet.payload {
        println!("SourceAddr = {}", payload.source_addr);
        assert_eq!(payload.source_addr, "935016");

        let timestamp: u32 = payload.timestamp.into();
        println!("Timestamp = {:010}", timestamp);
        assert_eq!(timestamp, 3_29_111740);
    }
}
// #endregion

// #region: CMPP_CONNECT_RESP
#[test]
fn test_encode_connect_response() {
    const LENGTH: usize = (LENGTH_HEADER + LENGTH_CONNECT_RESP) as usize;
    let mut encoder = PacketCodec::default();

    let packet = Packet {
        header: Header {
            total_length: LENGTH as u32,
            command_id: CMPP_CONNECT_RESP,
            sequence_id: 0x01_23_45_67,
        },
        payload: Payload::from(ConnectResponsePayload {
            status: 0x00,
            authenticator_ismg: [0u8; 16],
            version: 0x20.into(),
        }),
    };

    let mut dst = BytesMut::with_capacity(LENGTH);
    let _ = encoder.encode(packet, &mut dst);
    let bytes = &dst[..LENGTH];
    println!("{}", format_hex(&bytes, " ", true));
}

#[test]
fn test_decode_connect_response() {
    const LENGTH: usize = (LENGTH_HEADER + LENGTH_CONNECT_RESP) as usize;
    let mut decoder = PacketCodec::default();
    let mut src = BytesMut::with_capacity(LENGTH);
    src.put_slice(&hex_literal::hex!(
        r"
        00 00 00 1E 
        80 00 00 01 
        01 23 45 67 
        00 
        00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
        20 
        "
    ));

    let result = decoder.decode(&mut src);
    assert!(result.is_ok());
    let option = result.unwrap();
    assert!(option.is_some());
    let packet = option.unwrap();

    println!("报文长度：{0:} (0x{0:08x})", packet.header.total_length);
    assert_eq!(packet.header.total_length, 0x00_00_00_1E);

    println!("命令类型：{0:} (0x{0:08x})", packet.header.command_id);
    assert_eq!(packet.header.command_id, CMPP_CONNECT_RESP);

    println!("消息流水：{0:} (0x{0:08x})", packet.header.sequence_id);
    assert_eq!(packet.header.sequence_id, 0x01_23_45_67);

    if let Payload::ConnectResponse(payload) = packet.payload {
        println!("Status = 0x{:02x}", payload.status);
        assert_eq!(payload.status, 0x00);

        let version: u8 = payload.version.clone().into();
        println!("Version = 0x{:02x}", version);
        assert_eq!(version, 0x20);
    }
}
// #endregion

// #region: CMPP_SUBMIT
#[test]
fn test_encode_submit_request() {
    let mut encoder = PacketCodec::default();

    let packet = Packet {
        header: Header {
            total_length: 12 + 0,
            command_id: CMPP_SUBMIT,
            sequence_id: 0x01_23_45_80,
        },
        payload: Payload::from(SubmitRequestPayload {
            ..SubmitRequestPayload::default()
        }),
    };
}

#[test]
fn test_decode_submit_request() {
    //
}
// #endregion
