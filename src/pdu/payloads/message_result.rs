use crate::pdu::message_id::MessageId;

#[derive(Debug)]
pub struct MessageResultPayload {
    pub msg_id: MessageId,
    pub result: u8,
}
