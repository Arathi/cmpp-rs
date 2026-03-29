use crate::pdu::message_id::MessageId;

#[derive(Debug)]
pub struct DeliverRequestPayload {
    pub msg_id: MessageId,
    pub dest_id: String,
    pub service_id: String,
    pub tp_pid: u8,
    pub tp_udhi: u8,
    pub msg_fmt: u8,
    pub src_terminal_id: String,
    pub registered_delivery: u8,
    pub msg_length: u8,
    pub msg_content: Vec<u8>,
    pub reserved: [u8; 8],
}

#[derive(Debug)]
pub struct StatusReport {
    pub msg_id: MessageId,
    pub stat: String,
    pub submit_time: String,
    pub done_time: String,
    pub dest_terminal_id: String,
    pub smsc_sequence: [u8; 4],
}

#[derive(Debug)]
pub struct LongMesssage {
    pub remains_1: u8,
    pub magic: u8,
    pub remains_2: u8,
    pub batch: u16,
    pub index: u8,
    pub total: u8,
}
