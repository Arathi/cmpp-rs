use crate::pdu::command::Command;

#[derive(Debug)]
pub struct Header {
    pub total_length: u32,
    pub command_id: u32,
    pub sequence_id: u32,
}

impl Header {
    pub fn command(&self) -> Command {
        self.command_id.into()
    }
}
