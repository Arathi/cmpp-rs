pub struct SubmitRequest {
    msg_id: u64,
    pk_total: u8,
    pk_number: u8,
    registered_delivery: u8,
    msg_level: u8,
    service_id: String,
    fee_user_type: u8,
    fee_terminal_id: String,
    tp_pid: u8,
    tp_udhi: u8,
    msg_fmt: u8,
    msg_src: String,
    fee_type: String,
    fee_code: String,
    valid_time: String,
    at_time: String,
    src_id: String,
    dest_usr_tl: u8,
    dest_terminal_id: Vec<String>,
    msg_length: u8,
    msg_content: Vec<u8>,
    reserve: String,
}

pub struct SubmitResponse {
    msg_id: u64,
    result: u8,
}
