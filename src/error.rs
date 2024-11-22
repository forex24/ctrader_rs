use std::fmt::Debug;

use tokio::io;

use crate::protos::spotware_message::{ProtoErrorRes, ProtoOaErrorRes};

/// Fallible result values returned by the library.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors returned by the library.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The client is disconnected.")]
    Disconnected,

    #[error("IoTask Canceled")]
    Cancel,

    #[error("IoTask Join Error")]
    JoinError,
    //#[error("An error represented by an implementation of std::error::Error.")]
    //StdError(Box<dyn std::error::Error + Send + Sync>),
    #[error("An error represented as a String {0}.")]
    String(String),

    #[error("At least {0} more bytes required to frame packet")]
    InsufficientBytes(usize),

    #[error("payload size limit exceeded: {0}")]
    PayloadSizeLimitExceeded(usize),

    #[error("Parse Frame LengthError error")]
    ParseFrameLengthError,

    #[error("Decode ProtoMessage error")]
    DecodeProtoMessageError,

    #[error("Requests done")]
    RequestsDone,

    #[error("Io error: {0:?}")]
    Io(#[from] io::Error),
    #[error("ProtoMessage Serialization error")]
    Serialization,
    #[error("NotifySender error")]
    NotifySender,
    #[error("CtrlEventSender error")]
    CtrlEventSender,
    #[error("server report error: {0}")]
    ServerErrorRes(ProtoErrorRes),
    #[error("InvalidDnsNameError {0}")]
    DNSName(String),

    #[error("socket disconnect")]
    Disconnect,

    #[error("Timeout: {0}")]
    TimeoutError(u64),
    #[error("Internal send tx error is_hisorical {0}")]
    InternalSenderError(bool),
    #[error("Spotware error: {0}")]
    SpotwareError(ProtoOaErrorRes),

    #[error("Need Spotware version > {0}, but server version is {1}")]
    SpotwareVersionError(u32, u32),

    //
    #[error("server version parse int error: {0}")]
    ParseVersionError(String),

    //
    #[error("Wrong period: {0}")]
    PeriodParamError(i32),
}
