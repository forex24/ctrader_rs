use crate::credentials::{AccountCredentials, ApplicationCredentials};

#[derive(Clone)]
pub struct SessionConfig {
    pub application_credentials: ApplicationCredentials,
    pub account_credentials: AccountCredentials,
    pub server_url: String,
}

impl SessionConfig {
    pub fn load_from_env() -> Self {
        let server_url = std::env::var("server_url").expect("miss server addr in env");
        let application_credentials = ApplicationCredentials::load_from_env();
        let account_credentials = AccountCredentials::load_from_env();
        Self {
            application_credentials,
            account_credentials,
            server_url,
        }
    }
}
