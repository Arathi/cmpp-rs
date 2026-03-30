use crate::pdu::Command;
use crate::pdu::Header;
use crate::pdu::Payload;

#[derive(Debug)]
pub struct Packet {
    pub header: Header,
    pub payload: Payload,
}

impl Packet {
    pub fn total_length(&self) -> u32 {
        self.header.total_length
    }

    pub fn command_id(&self) -> u32 {
        self.header.command_id
    }

    pub fn sequence_id(&self) -> u32 {
        self.header.sequence_id
    }

    pub fn command(&self) -> Command {
        self.header.command()
    }
}
