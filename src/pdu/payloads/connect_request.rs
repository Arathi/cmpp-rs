use md5::{Digest, Md5};

use super::super::timestamp::Timestamp;
use super::super::version::Version;

#[derive(Debug)]
pub struct ConnectRequestPayload {
    // 6
    pub source_addr: String,
    // 16
    pub authenticator_source: [u8; 16],
    // 1
    pub version: Version,
    // 4
    pub timestamp: Timestamp,
}

impl ConnectRequestPayload {
    pub fn generate_authenticator_source<S>(&mut self, shared_secret: S)
    where
        S: Into<String>,
    {
        let timestamp: u32 = self.timestamp.clone().into();
        let hash = ConnectRequestPayload::calculate_authenticator_source(
            &self.source_addr,
            shared_secret,
            timestamp,
        );
        self.authenticator_source = hash;
    }

    pub fn calculate_authenticator_source<SA, SS, TS>(
        source_addr: SA,
        shared_secret: SS,
        timestamp: TS,
    ) -> [u8; 16]
    where
        SA: Into<String>,
        SS: Into<String>,
        TS: Into<u32>,
    {
        let input = format!(
            "{}000000000{}{:010}",
            source_addr.into(),
            shared_secret.into(),
            timestamp.into(),
        );
        let mut hasher = Md5::new();
        hasher.update(input.as_bytes());
        let hash: [u8; 16] = hasher.finalize().into();
        hash
    }

    // pub fn decode(src: &mut BytesMut) -> Option<ConnectRequestPayload> {
    //     let mut source_addr_bytes = [0u8; 6];
    //     src.copy_to_slice(&mut source_addr_bytes);
    //     let source_addr = String::from_utf8_lossy(&source_addr_bytes).to_string();
    //
    //     let mut authenticator_source = [0u8; 16];
    //     src.copy_to_slice(&mut authenticator_source);
    //
    //     let version_value = src.get_u8();
    //     let timestamp_value = src.get_u32();
    //
    //     Some(ConnectRequestPayload {
    //         source_addr,
    //         authenticator_source,
    //         version: version_value.into(),
    //         timestamp: timestamp_value.into(),
    //     })
    // }
}

// impl From<[u8; 27]> for ConnectRequestPayload {
//     fn from(value: [u8; 27]) -> Self {
//         let mut buf = BytesMut::with_capacity(27);
//         buf.copy_from_slice(&value);
//
//         let mut source_addr_bytes = [0u8; 6];
//         buf.copy_to_slice(&mut source_addr_bytes);
//         let source_addr = String::from_utf8_lossy(&source_addr_bytes).to_string();
//
//         let mut authenticator_source = [0u8; 16];
//         buf.copy_to_slice(&mut authenticator_source);
//
//         let version_byte = buf.get_u8();
//         let timestamp_u32 = buf.get_u32();
//
//         Self {
//             source_addr,
//             authenticator_source,
//             version: version_byte.into(),
//             timestamp: timestamp_u32.into(),
//         }
//     }
// }

// impl Into<[u8; 27]> for ConnectRequestPayload {
//     fn into(self) -> [u8; 27] {
//         let mut buf = BytesMut::with_capacity(27);
//
//         buf.put_slice(self.source_addr.as_bytes());
//         buf.put_slice(&self.authenticator_source);
//         buf.put_u8(self.version.into());
//         buf.put_u32(self.timestamp.into());
//
//         [0u8; 27]
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test() {
//         let mut payload = ConnectRequestPayload {
//             source_addr: "935016".into(),
//             authenticator_source: [0u8; 16],
//             version: 0x20.into(),
//             timestamp: Timestamp::now(),
//         };
//         let shared_secret = "pAs5W0rD";
//         payload.generate_authenticator_source(shared_secret);
//         println!("{:?}", payload);
//         println!(
//             "AuthenticatorSource = {}",
//             hex::encode(payload.authenticator_source).to_uppercase()
//         );
//     }
// }
