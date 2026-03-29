use crate::pdu::header::Header;
use crate::pdu::payloads::{
    ConnectRequestPayload, ConnectResponsePayload, DeliverRequestPayload, MessageResultPayload,
    SubmitRequestPayload,
};

#[derive(Debug)]
pub struct Message<P> {
    pub header: Header,
    pub payload: P,
}
