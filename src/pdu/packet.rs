use crate::pdu::Command;
use crate::pdu::Header;
use crate::pdu::Payload;

#[derive(Debug)]
pub struct Packet {
    pub header: Header,
    pub payload: Payload,
}

impl Packet {
    fn total_length(&self) -> u32 {
        self.header.total_length
    }

    fn command_id(&self) -> u32 {
        self.header.command_id
    }

    fn sequence_id(&self) -> u32 {
        self.header.sequence_id
    }

    fn command(&self) -> Command {
        self.header.command()
    }
}
