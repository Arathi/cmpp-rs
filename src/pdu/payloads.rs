mod connect_request;
mod connect_response;
mod deliver_request;
mod message_result;
mod submit_request;

pub use connect_request::ConnectRequestPayload;
pub use connect_response::ConnectResponsePayload;
pub use deliver_request::DeliverRequestPayload;
pub use message_result::MessageResultPayload;
pub use submit_request::SubmitRequestPayload;

#[derive(Debug)]
pub enum Payload {
    ConnectRequest(ConnectRequestPayload),
    ConnectResponse(ConnectResponsePayload),
    TerminateRequest,
    TerminateResponse,
    SubmitRequest(SubmitRequestPayload),
    SubmitResponse(MessageResultPayload),
    DeliverRequest(DeliverRequestPayload),
    DeliverResponse(MessageResultPayload),
    ActiveTestRequest,
    ActiveTestResponse(u8),
    Empty,
    MessageResult(MessageResultPayload),
}

impl From<ConnectRequestPayload> for Payload {
    fn from(value: ConnectRequestPayload) -> Self {
        Payload::ConnectRequest(value)
    }
}

impl From<ConnectResponsePayload> for Payload {
    fn from(value: ConnectResponsePayload) -> Self {
        Payload::ConnectResponse(value)
    }
}

impl From<SubmitRequestPayload> for Payload {
    fn from(value: SubmitRequestPayload) -> Self {
        Payload::SubmitRequest(value)
    }
}

impl From<DeliverRequestPayload> for Payload {
    fn from(value: DeliverRequestPayload) -> Self {
        Payload::DeliverRequest(value)
    }
}

impl From<MessageResultPayload> for Payload {
    fn from(value: MessageResultPayload) -> Self {
        Payload::MessageResult(value)
    }
}
