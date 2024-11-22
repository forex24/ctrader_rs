//! CTrader Client Builder Pattern
use crate::{
    credentials::{AccountCredentials, ApplicationCredentials},
    error::Error,
    error::Result,
};

use rustls::{ClientConfig, OwnedTrustAnchor, RootCertStore};
use tracing::warn;
use url::Url;

use ::rustls;
use std::sync::Arc;
use tokio::time::Duration;

use super::{client::Session, io::ConnectionMode, io::IoOptions};

/// A fluent builder interface to configure a Client.
///
/// Note that you must call `.set_url_string()` or `.set_url()` to configure a host and port to
/// connect to before `.build()`
#[derive(Default)]
pub struct ClientBuilder {
    url: Option<Url>,
    server_keep_alive: Option<Duration>,
    client_keep_alive: Option<Duration>,
    max_packet_len: Option<usize>,
    io_timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
    connection_mode: ConnectionMode,
    automatic_connect: Option<bool>,
    connect_retry_delay: Option<Duration>,
    application_credentials: Option<ApplicationCredentials>,
    account_credentials: Option<AccountCredentials>,
}

impl ClientBuilder {
    /// Build a new `Client` with this configuration.
    pub fn build(&mut self) -> Result<Session> {
        let opts = IoOptions {
            url: self
                .url
                .clone()
                .ok_or(Error::String("You must set a url for the client".into()))?,
            server_keep_alive: self.server_keep_alive.unwrap_or(Duration::from_secs(30)),
            client_keep_alive: self.client_keep_alive.unwrap_or(Duration::from_secs(10)),
            max_packet_len: self.max_packet_len.unwrap_or(1024 * 1024),
            io_timeout: self.io_timeout.unwrap_or(Duration::from_secs(5)),
            connect_timeout: self.connect_timeout.unwrap_or(Duration::from_secs(10)),
            connection_mode: self.connection_mode.clone(),
            automatic_connect: self.automatic_connect.unwrap_or(true),
            connect_retry_delay: self.connect_retry_delay.unwrap_or(Duration::from_secs(5)),
        };
        let app = self.application_credentials.clone().ok_or(Error::String(
            "You must set a application credentials for the client".into(),
        ))?;
        let account = self.account_credentials.clone().ok_or(Error::String(
            "You must set a account credential for the client".into(),
        ))?;
        Ok(Session::new(app, account, opts))
    }

    pub fn set_url_string(&mut self, url: &str) -> Result<&mut Self> {
        let url = Url::try_from(url).map_err(|e| Error::String(e.to_string()))?;
        self.set_url(url)
    }

    pub fn set_url(&mut self, url: Url) -> Result<&mut Self> {
        let rustls_config = default_tls_config();

        match url.scheme() {
            "tls" => {}
            _ => warn!(
                "only support server_url scheme is tls, ignore {}",
                url.scheme()
            ),
        }

        self.connection_mode = ConnectionMode::Tls(rustls_config);

        self.url = Some(url);
        Ok(self)
    }

    pub fn set_application_credentials(&mut self, app_cert: ApplicationCredentials) -> &mut Self {
        self.application_credentials = Some(app_cert);
        self
    }

    pub fn set_account_credentials(&mut self, account_cert: AccountCredentials) -> &mut Self {
        self.account_credentials = Some(account_cert);
        self
    }

    pub fn set_server_keep_alive(&mut self, keep_alive: Duration) -> &mut Self {
        self.server_keep_alive = Some(keep_alive);
        self
    }

    pub fn set_client_keep_alive(&mut self, keep_alive: Duration) -> &mut Self {
        self.client_keep_alive = Some(keep_alive);
        self
    }

    /// Set the maximum packet length.
    ///
    /// The default is 1024 * 1024 bytes.
    pub fn set_max_packet_len(&mut self, max_packet_len: usize) -> &mut Self {
        self.max_packet_len = Some(max_packet_len);
        self
    }

    /// Set the timeout for operations.
    ///
    /// The default is 5 seconds.
    pub fn set_io_timeout(&mut self, io_timeout: Duration) -> &mut Self {
        self.io_timeout = Some(io_timeout);
        self
    }

    /// Set the timeout for operations.
    ///
    /// The default is 5 seconds.
    pub fn set_connect_timeout(&mut self, connect_timeout: Duration) -> &mut Self {
        self.connect_timeout = Some(connect_timeout);
        self
    }

    /// Set the TLS ClientConfig for the client-server connection.
    ///
    /// Enables TLS. By default TLS is enabled.
    pub fn set_tls_client_config(&mut self, tls_client_config: rustls::ClientConfig) -> &mut Self {
        match self.connection_mode {
            ConnectionMode::Tls(ref mut config) => *config = Arc::new(tls_client_config),
        }
        self
    }

    /// Set whether to automatically connect and reconnect.
    ///
    /// The default is true.
    pub fn set_automatic_connect(&mut self, automatic_connect: bool) -> &mut Self {
        self.automatic_connect = Some(automatic_connect);
        self
    }

    /// Set the delay between connect retries.
    ///
    /// The default is 5s.
    pub fn set_connect_retry_delay(&mut self, connect_retry_delay: Duration) -> &mut Self {
        self.connect_retry_delay = Some(connect_retry_delay);
        self
    }
}

fn default_tls_config() -> Arc<ClientConfig> {
    let mut root_cert_store = RootCertStore::empty();
    root_cert_store.add_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.iter().map(|ta| {
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));

    let tls_config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();
    Arc::new(tls_config)
}
