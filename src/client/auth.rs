use tracing::{error, trace};

use super::Session;
use crate::{protos::spotware_message::*, Error};

impl Session {
    //+------------------------------------------------------------------+
    //|                               Auth                               |
    //+------------------------------------------------------------------+

    pub(crate) async fn refresh_token_and_reauth(&mut self) -> Result<(), Error> {
        trace!("refresh_token_and_reauth");
        // 1. For refreshing token you have to use the ProtoOARefreshTokenReq message
        // 2. After updating the refresh token you have to re-authorize the trading accounts by sending new ProtoOAAccountAuthReq message
        //    and you have to use the new token for all future requests
        // 3. You can use the token expiry time to check for expiry, and if your token expired
        //    you will receive a ProtoOAAccountsTokenInvalidatedEvent message
        let r = self.refresh_token_req(&self.account.refresh_token).await;
        match r {
            Err(err) => {
                error!(
                    "Refresh token request failed, you must restart client by manual\n reason: {}",
                    err
                );
                return Err(err);
            }
            Ok(res) => {
                self.account.access_token = res.access_token.to_string();
                self.account.refresh_token = res.refresh_token.to_string();
                self.account.expires_in = res.expires_in as u64;
                self.account.token_type = res.token_type.to_string();
            }
        }
        self.auth_account().await
    }

    // Request for getting the proxy version.
    // Can be used to check the current version of the Open API scheme.
    pub(crate) async fn get_server_version(&self) -> Result<u32, Error> {
        let req = ProtoOaVersionReq { payload_type: None };
        let res = self
            .connection
            .send_request(req.into())
            .await
            .map(ProtoOaVersionRes::from)?;

        let version = res
            .version
            .parse::<u32>()
            .map_err(|_| Error::ParseVersionError(res.version))?;
        Ok(version)
    }

    // Request for the authorizing an application to work with the cTrader platform Proxies.
    pub async fn auth_application(&self) -> Result<(), Error> {
        trace!("auth_application");
        let req = ProtoOaApplicationAuthReq {
            payload_type: None,
            client_id: self.application.client_id.clone(),
            client_secret: self.application.client_secret.clone(),
        };
        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaApplicationAuthRes::from)?;
        Ok(())
    }

    // Request for the authorizing trading account session.
    // Requires established authorized connection with the client application
    // using ProtoOAApplicationAuthReq.
    pub async fn auth_account(&self) -> Result<(), Error> {
        trace!("auth_account");
        let req = ProtoOaAccountAuthReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            access_token: self.account.access_token.clone(),
        };
        let m = self
            .connection
            .send_request(req.into())
            .await
            .map(ProtoOaAccountAuthRes::from)?;

        if self.account.account_id != m.ctid_trader_account_id {
            error!(
                "auth_account {} != {}",
                self.account.account_id, m.ctid_trader_account_id
            );
        }

        Ok(())
    }

    // Request for logout of trading account session
    pub async fn account_logout_req(&self) -> Result<(), Error> {
        let req = ProtoOaAccountLogoutReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
        };
        let m = self
            .connection
            .send_request(req.into())
            .await
            .map(ProtoOaAccountLogoutRes::from)?;

        if self.account.account_id != m.ctid_trader_account_id {
            error!(
                "auth_account {} != {}",
                self.account.account_id, m.ctid_trader_account_id
            );
        }

        Ok(())
    }

    //+------------------------------------------------------------------+
    //|                        Account&Token                             |
    //+------------------------------------------------------------------+

    // Request to refresh the access token using refresh token of granted trader's account.
    // 1. For refreshing token you have to use the ProtoOARefreshTokenReq message
    // 2. After updating the refresh token you have to re-authorize the trading accounts by sending new ProtoOAAccountAuthReq message
    //    and you have to use the new token for all future requests
    // 3. You can use the token expiry time to check for expiry, and if your token expired you will receive
    //    a ProtoOAAccountsTokenInvalidatedEvent message
    pub async fn refresh_token_req(
        &self,
        refresh_token: &str,
    ) -> Result<ProtoOaRefreshTokenRes, Error> {
        let req = ProtoOaRefreshTokenReq {
            payload_type: None,
            refresh_token: refresh_token.to_string(),
        };

        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaRefreshTokenRes::from)
    }
}
