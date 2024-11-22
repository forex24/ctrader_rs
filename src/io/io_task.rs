/// 核心IO任务，长期运行
use std::{mem, pin::Pin};

use futures::{SinkExt, StreamExt};
use governor::Quota;
use nonzero_ext::nonzero;
use tokio::{
    io::AsyncWriteExt,
    net::TcpStream,
    sync::{mpsc, oneshot},
    time::{self, sleep, timeout, Instant, Sleep},
};
use tokio_rustls::client::TlsStream;
use tokio_util::codec::Framed;
use tracing::{debug, error};

use crate::error::Error;

use super::{
    cm::connect_stream,
    codec::MsgCodec,
    options::IoOptions,
    processor::MessageProcessor,
    ratelimit::RateLimitUnboundedReceiver,
    types::{ConnectionState, Request, Response},
};

pub type MessageStream = Framed<TlsStream<TcpStream>, MsgCodec>;

/// The state held by the IO task, a long-running tokio future. The IO
/// task manages the underlying Tls/TCP connection, sends periodic
/// keep-alive heartbeat packets, and sends response packets to tasks that
/// are waiting.
pub struct IoTask {
    options: IoOptions,

    /// Request stream
    pub requests_rx: RateLimitUnboundedReceiver<Request>,
    /// Historical stream
    pub historical_rx: RateLimitUnboundedReceiver<Request>,

    /// client heartbeat time
    pub(crate) client_heartbeat_timeout: Option<Pin<Box<Sleep>>>,

    /// Keep alive time for server heartbeat
    pub(crate) server_heartbeat_timeout: Option<Pin<Box<Sleep>>>,

    /// Message processor, like incoming/outgoing message process etc.
    processor: MessageProcessor,

    /// enum value describing the current state as disconnected or connected.
    state: IoTaskState,

    /// Signal to the IO task to shutdown. Shared with IoTaskHandle.
    //halt: Arc<AtomicBool>,
    cancel_rx: oneshot::Receiver<()>,

    first_connect_tx: Option<oneshot::Sender<()>>,
}

#[derive(Debug)]
pub enum IoTaskState {
    Halted,
    Disconnected,
    Connected(MessageStream),
}

impl IoTask {
    pub fn new(
        options: IoOptions,
        requests_rx: mpsc::UnboundedReceiver<Request>,
        historical_rx: mpsc::UnboundedReceiver<Request>,
        responses_tx: mpsc::UnboundedSender<Response>,
        ctrl_tx: mpsc::UnboundedSender<ConnectionState>,
        cancel_rx: oneshot::Receiver<()>,
        first_connect_tx: oneshot::Sender<()>,
    ) -> Self {
        // see opan api document, 50/s for request, 5/s for historical
        let request_quota = Quota::per_second(nonzero!(50u32)).allow_burst(nonzero!(50u32));
        let historical_quota = Quota::per_second(nonzero!(5u32)).allow_burst(nonzero!(5u32));

        Self {
            options,
            processor: MessageProcessor::new(responses_tx, ctrl_tx),
            requests_rx: RateLimitUnboundedReceiver::new(requests_rx, request_quota),
            historical_rx: RateLimitUnboundedReceiver::new(historical_rx, historical_quota),
            client_heartbeat_timeout: None,
            server_heartbeat_timeout: None,
            state: IoTaskState::Disconnected,
            cancel_rx,
            first_connect_tx: Some(first_connect_tx),
        }
    }

    pub async fn run(mut self) {
        loop {
            match self.state {
                IoTaskState::Halted => return,
                IoTaskState::Disconnected => match Self::try_connect(&mut self).await {
                    Err(e) => {
                        error!("IoTask: Error connecting: {}", e);
                        if self.options.automatic_connect {
                            sleep(self.options.connect_retry_delay).await;
                        } else {
                            debug!(
                                "IoTask: halting due to connection failure, auto connect is off."
                            );
                            self.state = IoTaskState::Halted;
                            return;
                        }
                    }
                    Ok(()) => {
                        let first_connect_tx = self.first_connect_tx.take();
                        if let Some(sender) = first_connect_tx {
                            let _ = sender.send(());
                        }
                        let _ = self.processor.handle_on_connected();
                    }
                },
                IoTaskState::Connected(_) => match Self::run_once(&mut self).await {
                    Err(Error::Cancel) => {
                        debug!("IoTask: halting by request.");
                        self.shutdown_conn().await;
                        self.state = IoTaskState::Halted;
                    }
                    Err(Error::Disconnected) => {
                        debug!("IoTask: Disconnected, resetting state");
                        self.shutdown_conn().await;
                    }
                    Err(e) => {
                        error!("IoTask: Quitting run loop due to error: {}", e);
                        self.shutdown_conn().await;
                    }
                    _ => {}
                },
            }
        }
    }

    async fn try_connect(&mut self) -> Result<(), Error> {
        let stream = timeout(self.options.connect_timeout, connect_stream(&self.options))
            .await
            .map_err(|_| Error::TimeoutError(self.options.connect_timeout.as_secs()))??;
        let framed = Framed::new(stream, MsgCodec::default());
        self.state = IoTaskState::Connected(framed);
        Ok(())
    }

    async fn shutdown_conn(&mut self) {
        debug!("IoTask: shutdown_conn");

        // do clean
        self.server_heartbeat_timeout = None;
        self.client_heartbeat_timeout = None;

        let state = mem::replace(&mut self.state, IoTaskState::Disconnected);
        let framed = match state {
            // Already disconnected / halted, nothing more to do.
            IoTaskState::Disconnected | IoTaskState::Halted => return,
            IoTaskState::Connected(c) => c,
        };

        let _ = self.processor.handle_on_disconnected();

        let _ = framed.into_inner().shutdown().await;
    }

    /// Process on network and requests and generate keepalive pings when necessary
    async fn run_once(&mut self) -> Result<(), Error> {
        let framed = match self.state {
            IoTaskState::Connected(ref mut n) => n,
            _ => return Err(Error::String("Are you connect to network".to_string())),
        };

        if self.server_heartbeat_timeout.is_none() {
            self.server_heartbeat_timeout =
                Some(Box::pin(time::sleep(self.options.server_keep_alive)));
        }

        if self.client_heartbeat_timeout.is_none() {
            // 考虑到网络延时，客户端心跳间隔至少减1秒
            self.client_heartbeat_timeout = Some(Box::pin(time::sleep(
                self.options.client_keep_alive - std::time::Duration::from_secs(2),
            )));
        }

        tokio::select! {
            // Pull a bunch of packets from network, reply in bunch and yield the first item
            Some(o) = framed.next() => {
                match o {
                    Err(e) => return Err(Error::Io(e)),
                    Ok(m) => self.processor.handle_incoming_packet(m)?,
                }

                Ok(())
            },
            // Pull next request from user requests channel.
            o = self.requests_rx.recv() => {
                match o {
                    None => Err(Error::RequestsDone), // tx droped
                    Some(request) => {
                        let m = self.processor.prepare_outgoing_packet(request);
                        framed.send(m).await?;
                        framed.flush().await?;
                        Ok(())
                }

            }},
            // Pull next request from user historical requests channel.
            o = self.historical_rx.recv() => {
                match o {
                    None => Err(Error::RequestsDone), // tx droped
                    Some(request) => {
                        let m =self.processor.prepare_outgoing_packet(request);
                        framed.send(m).await?;
                        framed.flush().await?;
                        Ok(())
                }

            }},
            // We generate heartbeat irrespective of network activity.
            _ = self.client_heartbeat_timeout.as_mut().unwrap() => {
                let timeout = self.client_heartbeat_timeout.as_mut().unwrap();
                // 考虑到网络延时，客户端心跳间隔至少减1秒
                timeout.as_mut().reset(self.processor.last_outgoing + self.options.client_keep_alive - std::time::Duration::from_secs(1));

                let m = self.processor.prepare_heartbeat_packet();
                framed.send(m).await?;
                framed.flush().await?;

                Ok(())
            }
            // If heartbeat from server timeout
            _ = self.server_heartbeat_timeout.as_mut().unwrap() => {
                let timeout = self.server_heartbeat_timeout.as_mut().unwrap();
                timeout.as_mut().reset(self.processor.last_incoming + self.options.server_keep_alive);

                let elapsed_secs = Instant::now().duration_since(self.processor.last_incoming).as_secs();
                // 考虑到网络延时，服务端心跳间隔至少加1秒
                if elapsed_secs > self.processor.heartbeat_interval_secs + 2 {  // +2 是目前网络环境经常多延时1秒
                    error!(
                        "keeplive interval {} large than {}",
                        elapsed_secs,
                        self.processor.heartbeat_interval_secs
                    );
                    return Err(Error::Disconnected);
                }
                Ok(())
            }
            // cancellation requests to stop the polling
            _ = &mut self.cancel_rx  => {
                Err(Error::Cancel)
            }
        }
    }
}
