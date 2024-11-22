use std::fmt;

use crate::protos::spotware_message::ProtoMessage;
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum ConnectionState {
    Connected,
    Disconnect,
}

#[derive(Debug)]
pub enum Event {
    Message(ProtoMessage),
    Control(ConnectionState),
}

#[derive(Debug)]
pub struct Response {
    pub message: ProtoMessage,
}

pub struct Request {
    pub message: ProtoMessage,
    pub tx: Option<oneshot::Sender<Response>>,
}

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Request")
            .field("msg", &self.message)
            .finish()
    }
}
