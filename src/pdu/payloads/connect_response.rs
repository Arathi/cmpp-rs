use crate::pdu::version::Version;

#[derive(Debug)]
pub struct ConnectResponsePayload {
    // 1
    pub status: u8,
    // 16
    pub authenticator_ismg: [u8; 16],
    // 1
    pub version: Version,
}

impl ConnectResponsePayload {
    pub fn generate_authenticator_ismg<S: Into<String>>(
        &mut self,
        authenticator_source: [u8; 16],
        shared_secret: S,
    ) {
        // let input = [
        //     status,
        //     ..authenticator_source,
        //     ..shared_secret,
        // ];
        self.authenticator_ismg = [0u8; 16];
    }

    pub fn calculate_authenticator_ismg() -> [u8; 16] {
        [0; 16]
    }
}
