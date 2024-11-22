/// Connection类主要是为了屏蔽一些底层信息
use std::{io, time::Duration};

use tokio::{
    sync::{mpsc, oneshot},
    task::JoinHandle,
    time::timeout,
};

use crate::{error::Error, protos::spotware_message::*};

use super::io_task::IoTask;
use super::options::IoOptions;
use super::types::{ConnectionState, Event, Request, Response};

#[derive(Debug)]
struct IoTaskHandle {
    pub(crate) requests_tx: mpsc::UnboundedSender<Request>,
    pub(crate) historical_tx: mpsc::UnboundedSender<Request>,
    pub(crate) responses_rx: mpsc::UnboundedReceiver<Response>,
    pub(crate) ctrl_rx: mpsc::UnboundedReceiver<ConnectionState>,

    // Signal to the IO task to shutdown.
    cancel_tx: Option<tokio::sync::oneshot::Sender<()>>,
    // Join the IO task.
    join_handle: Option<JoinHandle<()>>,
}

#[derive(Debug)]
pub struct Connection {
    /// Options configured for the client
    options: IoOptions,
    /// Handle values to communicate with the IO task
    io_task_handle: Option<IoTaskHandle>,
}

impl Connection {
    pub fn new(opts: IoOptions) -> Self {
        Self {
            options: opts,
            io_task_handle: None,
        }
    }

    pub async fn connect(&mut self) -> Result<(), Error> {
        let rx = self.spawn_io_task()?;
        let _ = rx.await;
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<(), Error> {
        let c = self.check_io_task_mut()?;

        let tx = c.cancel_tx.take();

        if let Some(tx) = tx {
            let _ = tx.send(());
        }

        let join_handle = c.join_handle.take();
        if let Some(handle) = join_handle {
            let _ = handle.await.map_err(|_| Error::JoinError);
        }

        self.io_task_handle = None;

        Ok(())
    }

    #[inline]
    pub async fn send_request(&self, message: ProtoMessage) -> Result<ProtoMessage, Error> {
        self._send_request(message, false).await
    }

    #[inline]
    pub async fn send_historical_request(
        &self,
        message: ProtoMessage,
    ) -> Result<ProtoMessage, Error> {
        self._send_request(message, true).await
    }

    // 这个API设计要斟酌下
    pub async fn listen(&mut self) -> Option<Event> {
        let c = self.check_io_task_mut().unwrap();

        tokio::select! {
            ctrl_event = c.ctrl_rx.recv() => {
                if let Some(event) = ctrl_event {
                    return Some(Event::Control(event));
                }

            }
            response = c.responses_rx.recv() => {
                if let Some(msg) = response {
                    return Some(Event::Message(msg.message));
                }
            }
        }
        None
    }

    // Private API
    fn spawn_io_task(&mut self) -> Result<tokio::sync::oneshot::Receiver<()>, Error> {
        self.check_no_io_task()?;
        // 普通的request,20/s
        let (requests_tx, requests_rx) = mpsc::unbounded_channel();
        // 历史数据的request,5/s,由于速率限制，需要区分
        let (historical_tx, historical_rx) = mpsc::unbounded_channel();
        // 与long-run的io task 通讯的信道，把所有的请求发送给io task
        let (responses_tx, responses_rx) = mpsc::unbounded_channel();
        // 用于io task 发送connect/disconnect信息
        let (ctrl_tx, ctrl_rx) = mpsc::unbounded_channel();
        // 控制通道，用于停止io task的运行
        let (cancel_tx, cancel_rx) = tokio::sync::oneshot::channel::<()>();
        // 用于第一次链接用的通讯信道
        let (first_connect_tx, first_connect_rx) = tokio::sync::oneshot::channel::<()>();

        let io = IoTask::new(
            self.options.clone(),
            requests_rx,
            historical_rx,
            responses_tx,
            ctrl_tx,
            cancel_rx,
            first_connect_tx,
        );

        let handle = tokio::spawn(io.run());

        self.io_task_handle = Some(IoTaskHandle {
            requests_tx,
            historical_tx,
            responses_rx,
            ctrl_rx,
            cancel_tx: Some(cancel_tx),
            join_handle: Some(handle),
        });

        Ok(first_connect_rx)
    }

    fn check_io_task_mut(&mut self) -> Result<&mut IoTaskHandle, Error> {
        match self.io_task_handle {
            Some(ref mut h) => Ok(h),
            None => Err(Error::String("No IO task, did you call connect?".into())),
        }
    }

    fn check_io_task(&self) -> Result<&IoTaskHandle, Error> {
        match self.io_task_handle {
            Some(ref h) => Ok(h),
            None => Err(Error::String("No IO task, did you call connect?".into())),
        }
    }

    fn check_no_io_task(&self) -> Result<(), Error> {
        match self.io_task_handle {
            Some(_) => Err(Error::String("Already spawned IO task".into())),
            None => Ok(()),
        }
    }

    #[allow(dead_code)]
    pub async fn post_message(&self, message: ProtoMessage) -> Result<(), Error> {
        let c = self.check_io_task()?;
        let req = Request { message, tx: None };

        c.requests_tx
            .send(req)
            .map_err(|_| Error::InternalSenderError(false))
    }

    #[allow(dead_code)]
    pub async fn post_historical_message(&self, message: ProtoMessage) -> Result<(), Error> {
        let c = self.check_io_task()?;
        let req = Request { message, tx: None };

        c.historical_tx
            .send(req)
            .map_err(|_| Error::InternalSenderError(true))
    }

    async fn _send_request(
        &self,
        message: ProtoMessage,
        historical: bool,
    ) -> Result<ProtoMessage, Error> {
        let timeout = self.options.io_timeout;
        match self
            .timed_request(message, timeout.as_secs(), historical)
            .await
        {
            Ok(msg) => {
                if msg.payload_type == ProtoOaPayloadType::ProtoOaErrorRes as u32 {
                    Err(Error::SpotwareError(ProtoOaErrorRes::from(msg)))
                } else {
                    Ok(msg)
                }
            }
            Err(e) => Err(e),
        }
    }

    fn convert_error_response(msg: ProtoMessage) -> Result<ProtoMessage, Error> {
        if msg.payload_type == ProtoOaPayloadType::ProtoOaErrorRes as u32 {
            Err(Error::SpotwareError(ProtoOaErrorRes::from(msg)))
        } else {
            Ok(msg)
        }
    }

    async fn timed_request(
        &self,
        message: ProtoMessage,
        timeout_ms: u64,
        historical: bool,
    ) -> Result<ProtoMessage, Error> {
        let c = self.check_io_task()?;
        let (tx, rx) = oneshot::channel::<Response>();
        let req = Request {
            message,
            tx: Some(tx),
        };

        let request_tx = if historical {
            &c.historical_tx
        } else {
            &c.requests_tx
        };

        request_tx
            .send(req)
            .map_err(|_| Error::InternalSenderError(historical))?;

        if timeout_ms > 0 {
            let r = timeout(Duration::from_secs(timeout_ms), rx).await;
            match r {
                Ok(recv) => match recv {
                    Ok(response) => Self::convert_error_response(response.message),
                    Err(e) => Err(Error::from(io::Error::new(io::ErrorKind::BrokenPipe, e))),
                },
                Err(_) => Err(Error::TimeoutError(timeout_ms)),
            }
        } else {
            let r = rx
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::BrokenPipe, e))
                .map(|response| response.message);
            match r {
                Ok(msg) => Self::convert_error_response(msg),
                Err(e) => Err(Error::from(e)),
            }
        }
    }
}
