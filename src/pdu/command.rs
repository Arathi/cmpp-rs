pub const CMPP_CONNECT: u32 = 0x00_00_00_01;
pub const CMPP_TERMINATE: u32 = 0x00_00_00_02;
pub const CMPP_SUBMIT: u32 = 0x00_00_00_04;
pub const CMPP_DELIVER: u32 = 0x00_00_00_05;
pub const CMPP_ACTIVE_TEST: u32 = 0x00_00_00_08;
pub const CMPP_CONNECT_RESP: u32 = 0x80_00_00_01;
pub const CMPP_TERMINATE_RESP: u32 = 0x80_00_00_02;
pub const CMPP_SUBMIT_RESP: u32 = 0x80_00_00_04;
pub const CMPP_DELIVER_RESP: u32 = 0x80_00_00_05;
pub const CMPP_ACTIVE_TEST_RESP: u32 = 0x80_00_00_08;

pub const CONNECT: u8 = 0x01;
pub const TERMINATE: u8 = 0x02;
pub const SUBMIT: u8 = 0x04;
pub const DELIVER: u8 = 0x05;
pub const ACTIVE_TEST: u8 = 0x08;

#[derive(Debug, Clone, PartialEq)]
#[repr(u32)]
pub enum Command {
    Unknown = 0,
    ConnectRequest = CMPP_CONNECT,
    ConnectResponse = CMPP_CONNECT_RESP,
    TerminateRequest = CMPP_TERMINATE,
    TerminateResponse = CMPP_TERMINATE_RESP,
    SubmitRequest = CMPP_SUBMIT,
    SubmitResponse = CMPP_SUBMIT_RESP,
    DeliverRequest = CMPP_DELIVER,
    DeliverResponse = CMPP_DELIVER_RESP,
    ActiveTestRequest = CMPP_ACTIVE_TEST,
    ActiveTestResponse = CMPP_ACTIVE_TEST_RESP,
}

impl From<u32> for Command {
    fn from(value: u32) -> Self {
        match value {
            CMPP_CONNECT => Command::ConnectRequest,
            CMPP_CONNECT_RESP => Command::ConnectResponse,
            CMPP_TERMINATE => Command::TerminateRequest,
            CMPP_TERMINATE_RESP => Command::TerminateRequest,
            CMPP_SUBMIT => Command::SubmitRequest,
            CMPP_SUBMIT_RESP => Command::SubmitResponse,
            CMPP_DELIVER => Command::DeliverRequest,
            CMPP_DELIVER_RESP => Command::DeliverResponse,
            CMPP_ACTIVE_TEST => Command::ActiveTestRequest,
            CMPP_ACTIVE_TEST_RESP => Command::ActiveTestResponse,
            _ => Command::Unknown,
        }
    }
}

impl Into<u32> for Command {
    fn into(self: Self) -> u32 {
        match self {
            Command::ConnectRequest => CMPP_CONNECT,
            Command::ConnectResponse => CMPP_CONNECT_RESP,
            Command::TerminateRequest => CMPP_TERMINATE,
            Command::TerminateResponse => CMPP_TERMINATE_RESP,
            Command::SubmitRequest => CMPP_SUBMIT,
            Command::SubmitResponse => CMPP_SUBMIT_RESP,
            Command::DeliverRequest => CMPP_DELIVER,
            Command::DeliverResponse => CMPP_DELIVER_RESP,
            Command::ActiveTestRequest => CMPP_ACTIVE_TEST,
            Command::ActiveTestResponse => CMPP_ACTIVE_TEST_RESP,
            _ => 0,
        }
    }
}
