use super::types::{ConnectionState, Request, Response};
use crate::error::Error;
use crate::protos::spotware_message::*;

use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot};
use tokio::time::Instant;
use tracing::{error, trace};
use uuid::Uuid;

/// State of the connection.
#[derive(Debug)]
pub struct MessageProcessor {
    /// Last incoming packet time
    pub(crate) last_incoming: Instant,

    /// Last outgoing packet time
    pub(crate) last_outgoing: Instant,

    /// heartbeat interval, default is 30s, see open api document
    pub(crate) heartbeat_interval_secs: u64,

    // map client_msg_id to response
    store: HashMap<String, oneshot::Sender<Response>>,

    // spotware event tx
    responses_tx: mpsc::UnboundedSender<Response>,

    // socket connect/disconnect event tx
    ctrl_tx: mpsc::UnboundedSender<ConnectionState>,
}

impl MessageProcessor {
    pub fn new(
        responses_tx: mpsc::UnboundedSender<Response>,
        ctrl_tx: mpsc::UnboundedSender<ConnectionState>,
    ) -> Self {
        MessageProcessor {
            last_incoming: Instant::now(),
            last_outgoing: Instant::now(),
            heartbeat_interval_secs: 30,
            store: HashMap::new(),
            responses_tx,
            ctrl_tx,
        }
    }

    fn kv_get(&mut self, id: &String) -> Option<oneshot::Sender<Response>> {
        self.store.remove(id)
    }

    fn kv_set(&mut self, id: String, tx: oneshot::Sender<Response>) {
        self.store.insert(id, tx);
    }

    fn create_unique_id(&self) -> String {
        Uuid::new_v4().to_string()
    }

    pub fn handle_on_connected(&mut self) -> Result<(), Error> {
        trace!("handle_on_connected");
        self.ctrl_tx
            .send(ConnectionState::Connected)
            .map_err(|_| Error::CtrlEventSender)?;
        Ok(())
    }

    pub fn handle_on_disconnected(&mut self) -> Result<(), Error> {
        trace!("handle_on_disconnected");
        self.ctrl_tx
            .send(ConnectionState::Disconnect)
            .map_err(|_| Error::CtrlEventSender)?;
        Ok(())
    }

    /// Consolidates handling of all outgoing packet logic.
    pub fn prepare_outgoing_packet(&mut self, request: Request) -> ProtoMessage {
        trace!("handle_outgoing_packet");
        let mut req = request;
        if req.tx.is_some() {
            // tx.is_some means using request-response model
            let id = self.create_unique_id();
            req.message.client_msg_id = Some(id.clone());
            self.kv_set(id, req.tx.unwrap());
        }

        self.last_outgoing = Instant::now();

        trace!("Outgoing packet {:?}", req.message);

        req.message
    }

    /// Special case for heartbeat packet
    pub fn prepare_heartbeat_packet(&mut self) -> ProtoMessage {
        trace!("handle_heartbeat_packet");
        let msg = ProtoMessage::from(ProtoHeartbeatEvent::default());
        self.last_outgoing = Instant::now();
        msg
    }

    /// Consolidates handling of all incoming packet logic.
    pub fn handle_incoming_packet(&mut self, packet: ProtoMessage) -> Result<(), Error> {
        trace!("handle_incoming_packet {:?}", packet);
        self.last_incoming = Instant::now();

        if packet.payload_type == ProtoPayloadType::HeartbeatEvent as u32 {
            trace!("Heartbeat packet");
            return Ok(());
        }

        if packet.payload_type == ProtoPayloadType::ErrorRes as u32 {
            let error_packet = ProtoErrorRes::from(packet);
            error!("ErrorRes packet {}", error_packet);
            return Err(Error::ServerErrorRes(error_packet));
        }

        match packet.client_msg_id {
            None => {
                // 没有client_msg_id代表是Event，例如市场数据等
                let r = self.responses_tx.send(Response { message: packet });
                if r.is_err() {
                    return Err(Error::NotifySender);
                }
            }
            Some(ref client_msg_id) => {
                // 如果有client_msg_id代表是request-response模式，需要查找相应的request对应的oneshot
                let sender = match self.kv_get(client_msg_id) {
                    Some(s) => s,
                    None => {
                        error!("NotFoundId:{}", client_msg_id);
                        return Ok(());
                    }
                };

                let r = sender.send(Response { message: packet });
                if r.is_err() {
                    error!("OneshotSenderError");
                    return Ok(());
                }
            }
        }
        Ok(())
    }
}
