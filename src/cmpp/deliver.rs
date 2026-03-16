pub struct DeliverRequest {
    msg_id: u64,
    dest_id: String,
    service_id: String,
    tp_pid: u8,
    tp_udhi: u8,
    msg_fmt: u8,
    src_terminal_id: String,
    registered_delivery: u8,
    msg_length: u8,
    msg_content: Vec<u8>,
    reserved: String,
}

impl DeliverRequest {
    fn reply_content(&self) -> Option<String> {
        if self.registered_delivery != 0x00 {
            return None;
        }

        let content = "".to_string();
        return Some(content);
    }
    
    fn status_report(&self) -> Option<StatusReport> {
        if self.registered_delivery != 0x01 {
            return None;
        }

        let msg_id = 0;
        let stat = "DELIVRD".to_string();
        let submit_time = "".to_string();
        let done_time = "".to_string();
        let dest_terminal_id = "".to_string();
        let smsc_sequence = 0;
        return Some(StatusReport {
            msg_id,
            stat,
            submit_time,
            done_time,
            dest_terminal_id,
            smsc_sequence,
        });
    }
}

pub struct StatusReport {
    msg_id: u64,
    stat: String,
    submit_time: String,
    done_time: String,
    dest_terminal_id: String,
    smsc_sequence: u32,
}

pub struct LongMessage {
    length: u8,
    batch: u16,
    amount: u8,
    total: u8,
}

pub struct DeliverResponse {
    msg_id: u64,
    result: u8,
}

