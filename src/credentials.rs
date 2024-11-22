use std::env;

#[derive(Debug, Clone)]
pub struct AccountCredentials {
    pub account_id: i64,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
}
impl AccountCredentials {
    pub fn load_from_env() -> Self {
        let account_id = env::var("account_id")
            .expect("miss account_id in env")
            .parse::<i64>()
            .expect("parse account_id fail");
        let access_token = env::var("access_token").expect("miss access_token in env");
        let token_type = env::var("token_type").expect("miss token_type in env");
        let expires_in = env::var("expires_in")
            .expect("miss expires_in in env")
            .parse::<u64>()
            .expect("parse expires_in fail");
        let refresh_token = env::var("refresh_token").expect("miss refresh_token in env");
        Self {
            account_id,
            access_token,
            token_type,
            expires_in,
            refresh_token,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApplicationCredentials {
    pub client_id: String,
    pub client_secret: String,
}

impl ApplicationCredentials {
    pub fn load_from_env() -> Self {
        let client_id = env::var("client_id").expect("miss client_id in env");
        let client_secret = env::var("client_secret").expect("miss client_secret in env");
        Self {
            client_id,
            client_secret,
        }
    }
}
