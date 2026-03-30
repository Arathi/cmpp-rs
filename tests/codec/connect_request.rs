use actix_codec::{Decoder, Encoder};
use bytes::{BufMut, BytesMut};
use cmpp_lib::codec::*;
use cmpp_lib::pdu::command::CMPP_CONNECT;
use cmpp_lib::pdu::*;

#[test]
fn test_encode() {
    let mut encoder = PacketCodec::default();
    let shared_secret = "pAs5W0rD";
    let mut payload = ConnectRequestPayload {
        source_addr: "935016".to_string(),
        authenticator_source: [0u8; 16],
        version: 0x20.into(),
        timestamp: Timestamp::now(),
    };
    payload.generate_authenticator_source(shared_secret);
    let packet = Packet {
        header: Header {
            total_length: 0x00_00_00_27,
            command_id: CMPP_CONNECT,
            sequence_id: 0x00_01_00_00,
        },
        payload: Payload::from(payload),
    };
    let mut dst = BytesMut::new();
    let _ = encoder.encode(packet, &mut dst);
    println!("{}", hex::encode(&dst));
}

#[test]
fn test_decode() {
    let mut decoder = PacketCodec::default();
    let mut src = BytesMut::new();
    src.put_slice(&hex_literal::hex!(
        r#"
            00000027
            00000001
            00010000
            393335303136
            00000000000000000000000000000000
            20
            13acfedb
        "#
    ));

    let result = decoder.decode(&mut src);
    assert!(result.is_ok());

    let option = result.unwrap();
    assert!(option.is_some());

    let packet = option.unwrap();
    println!("TotalLength = 0x{0:08x} ({0:})", packet.total_length());
    assert_eq!(packet.total_length(), 0x00_00_00_27);

    println!(
        "CommandId = 0x{:08x} ({:?})",
        packet.command_id(),
        packet.command(),
    );
    assert_eq!(packet.command_id(), CMPP_CONNECT);

    println!("SequenceId = 0x{0:08x} ({0:})", packet.sequence_id());
    assert_eq!(packet.sequence_id(), 0x00_01_00_00);

    assert!(matches!(packet.payload, Payload::ConnectRequest(_)));

    match packet.payload {
        Payload::ConnectRequest(payload) => {
            println!("SourceAddr = {}", &payload.source_addr);
            assert_eq!(payload.source_addr, "935016");

            println!(
                "AuthenticatorSource = {}",
                hex::encode(&payload.authenticator_source)
            );
            assert_eq!(payload.authenticator_source, [0u8; 16]);

            let version_byte: u8 = payload.version.clone().into();
            println!("Version = 0x{:02x} ({})", version_byte, payload.version);
            assert_eq!(payload.version, 0x20);

            let timestamp_u32: u32 = payload.timestamp.clone().into();
            println!(
                "Timestamp = 0x{:08x} ({})",
                timestamp_u32, payload.timestamp
            );
            assert_eq!(payload.timestamp, 0x13acfedb);
        }
        _ => (),
    }
}
