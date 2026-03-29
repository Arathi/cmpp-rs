use crate::pdu::message_id::MessageId;

#[derive(Debug, Default)]
pub struct SubmitRequestPayload {
    pub msg_id: MessageId,
    pub pk_total: u8,
    pub pk_number: u8,
    pub registered_delivery: u8,
    pub msg_level: u8,
    pub service_id: String,
    pub fee_user_type: u8,
    pub fee_terminal_id: String,
    pub tp_pid: u8,
    pub tp_udhi: u8,
    pub msg_fmt: u8,
    pub msg_src: String,
    pub fee_type: String,
    pub fee_code: String,
    pub valid_time: String,
    pub at_time: String,
    pub src_id: String,
    pub dest_usr_tl: u8,
    pub dest_terminal_id: String,
    pub msg_length: u8,
    pub msg_content: Vec<u8>,
    pub reserve: [u8; 8],
}
