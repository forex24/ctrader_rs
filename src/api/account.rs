use chrono::{DateTime, Utc};


#[derive(Debug)]
pub struct Account {
    account_id: u64,
    is_live: bool,
    is_tradable: bool,
    trader_login: i64,
    last_closing_deal_timestamp: DateTime<Utc>,
    last_balance_update_timestamp: DateTime<Utc>,
}

impl Account {
    pub async fn auth() -> Self {

    }
}