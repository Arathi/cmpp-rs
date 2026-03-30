use actix_codec::{Decoder, Encoder};
use bytes::{BufMut, BytesMut};
use cmpp_lib::codec::*;
use cmpp_lib::pdu::command::CMPP_CONNECT_RESP;
use cmpp_lib::pdu::*;

#[test]
fn test_encode() {
    let mut encoder = PacketCodec::default();
    let authenticator_source = [0u8; 16];
    let shared_secret = "pAs5W0rD";
    let mut payload = ConnectResponsePayload {
        status: 0x00,
        authenticator_ismg: [0u8; 16],
        version: 0x20.into(),
    };

    payload.generate_authenticator_ismg(&authenticator_source, shared_secret);
    let packet = Packet {
        header: Header {
            total_length: 0x00_00_00_1E,
            command_id: CMPP_CONNECT_RESP,
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
            0000001e
            80000001
            00010000
            00
            00000000000000000000000000000000
            20
        "#
    ));

    let result = decoder.decode(&mut src);
    assert!(result.is_ok());

    let option = result.unwrap();
    assert!(option.is_some());

    let packet = option.unwrap();
    println!("TotalLength = 0x{0:08x} ({0:})", packet.total_length());
    assert_eq!(packet.total_length(), 0x00_00_00_1E);

    println!(
        "CommandId = 0x{:08x} ({:?})",
        packet.command_id(),
        packet.command(),
    );
    assert_eq!(packet.command_id(), CMPP_CONNECT_RESP);

    println!("SequenceId = 0x{0:08x} ({0:})", packet.sequence_id());
    assert_eq!(packet.sequence_id(), 0x00_01_00_00);

    assert!(matches!(packet.payload, Payload::ConnectResponse(_)));

    match packet.payload {
        Payload::ConnectResponse(payload) => {
            println!("Status = 0x{0:02x} ({0})", &payload.status);
            assert_eq!(payload.status, 0x00);

            println!(
                "AuthenticatorISMG = {}",
                hex::encode(&payload.authenticator_ismg)
            );
            assert_eq!(payload.authenticator_ismg, [0u8; 16]);

            let version_byte: u8 = payload.version.clone().into();
            println!("Version = 0x{:02x} ({})", version_byte, payload.version);
            assert_eq!(payload.version, 0x20);
        }
        _ => (),
    }
}
