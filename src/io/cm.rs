/// ConnectionManager
use rustls::{ClientConfig, OwnedTrustAnchor, RootCertStore, ServerName};
use std::{convert::TryFrom, sync::Arc};
use tokio::net::TcpStream;
use tokio_rustls::{client::TlsStream, TlsConnector};
use tracing::debug;

use crate::error::Error;

use super::options::IoOptions;

/// An enum for specifying which mode we will use to connect to the broker
#[derive(Clone)]
pub enum ConnectionMode {
    Tls(Arc<rustls::ClientConfig>),
}

impl Default for ConnectionMode {
    fn default() -> Self {
        Self::Tls(default_tls_config())
    }
}

async fn tls_connect(
    host: &str,
    port: u16,
    c: &Arc<ClientConfig>,
) -> Result<TlsStream<TcpStream>, Error> {
    let connector = TlsConnector::from(c.clone());
    let domain = ServerName::try_from(host).map_err(|e| Error::DNSName(e.to_string()))?;
    let tcp = TcpStream::connect((host, port)).await?;
    tcp.set_nodelay(true).expect("nodelay");
    let conn = connector.connect(domain, tcp).await?;
    debug!("tls connect ok");
    Ok(conn)
}

/// Start network connection to the server.
pub async fn connect_stream(opts: &IoOptions) -> Result<TlsStream<TcpStream>, Error> {
    debug!("Connecting to {}", opts.url);

    let host = opts
        .url
        .host_str()
        .ok_or(Error::String("Missing host".to_owned()))?;
    let port = opts.url.port().unwrap_or(5035);

    match opts.connection_mode {
        ConnectionMode::Tls(ref c) => {
            let conn = tls_connect(host, port, c).await?;
            Ok(conn)
        }
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
