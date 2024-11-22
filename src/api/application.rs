

#[derive(Debug)]
pub struct Application {
    Accounts: HashMap<i64, Account>,
    server_version: u32,
    user_id: i64,
}

impl Application {
    pub async fn get_account_list(client: &Client) {
        client.account_list_by_access_token()
    }

    pub async fn version() {}

    pub async fn get_ctid_profile() {
        // ProtoOACtidProfile¶
        // Field	Type	Label	Description
        // userId	int64	Required	
    }
}