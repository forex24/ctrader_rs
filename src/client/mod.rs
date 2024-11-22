use crate::builder::ClientBuilder;
use crate::credentials::AccountCredentials;
use crate::credentials::ApplicationCredentials;
use crate::io::Event;
use crate::io::IoOptions;
use crate::protos::spotware_message::ProtoMessage;
use crate::util::get_symbol_infos;
use crate::util::SymbolStore;
use crate::{error::Error, io::Connection};

#[allow(unused)]
const LIBRARY_IMPL_FOR_SERVER_VERSION: u32 = 88;

#[derive(Debug)]
pub struct Session {
    pub application: ApplicationCredentials,
    pub account: AccountCredentials,
    pub version: u32,
    connection: Connection,
    broadcast_tx: tokio::sync::broadcast::Sender<NotifyEvent>,
    subscribed_spots: Vec<i64>,
    subscribed_bars: Vec<(i32, i64)>,
    subscribed_depths: Vec<i64>,
    pub store: SymbolStore,
}

impl Session {
    // base function
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub(crate) fn new(
        application: ApplicationCredentials,
        account: AccountCredentials,
        opts: IoOptions,
    ) -> Self {
        let (tx, _) = tokio::sync::broadcast::channel(100);
        Self {
            application,
            account,
            version: 0,
            connection: Connection::new(opts),
            broadcast_tx: tx,
            subscribed_spots: Vec::new(),
            subscribed_bars: Vec::new(),
            subscribed_depths: Vec::new(),
            store: SymbolStore::new(),
        }
    }

    pub async fn connect(&mut self) -> Result<(), Error> {
        self.connection.connect().await?;
        self.version = self.get_server_version().await?;
        self.auth_application().await?;
        self.auth_account().await?;
        let infos = get_symbol_infos(self).await?;
        self.store.from_symbol_infos(&infos);
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<(), Error> {
        self.account_logout_req().await?;
        self.connection.shutdown().await?;
        Ok(())
    }

    pub fn server_version(&self) -> u32 {
        self.version
    }

    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<NotifyEvent> {
        self.broadcast_tx.subscribe()
    }

    pub async fn listen(&mut self) -> Option<Event> {
        self.connection.listen().await
    }

    pub async fn post_message(&self, message: ProtoMessage) -> Result<(), Error> {
        self.connection.post_message(message).await
    }

    pub async fn post_historical_message(&self, message: ProtoMessage) -> Result<(), Error> {
        self.connection.post_historical_message(message).await
    }
}

pub mod account;
pub mod auth;
pub mod event;
pub mod historical;
pub mod margin;
pub mod marketdata;
pub mod misc;
pub mod order;
pub mod position;
pub mod symbol;

pub use event::NotifyEvent;
