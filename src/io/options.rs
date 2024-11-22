use std::{fmt, time::Duration};

use url::Url;

use super::cm::ConnectionMode;

#[derive(Clone)]
pub struct IoOptions {
    // See ClientBuilder methods for per-field documentation.
    pub url: Url,
    pub connection_mode: ConnectionMode,
    pub server_keep_alive: Duration,
    pub client_keep_alive: Duration,
    pub max_packet_len: usize,
    pub io_timeout: Duration,
    pub connect_timeout: Duration,
    pub automatic_connect: bool,
    pub connect_retry_delay: Duration,
}

impl fmt::Debug for IoOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IoOptions")
            .field("url", &self.url)
            .field("connect_timeout", &self.connect_timeout)
            .field("server_keep_alive", &self.server_keep_alive)
            .field("client_keep_alive", &self.client_keep_alive)
            .field("max_packet_len", &self.max_packet_len)
            .field("io_timeout", &self.io_timeout)
            .field("automatic_connect", &self.automatic_connect)
            .field("connect_retry_delay", &self.connect_retry_delay)
            .finish()
    }
}
