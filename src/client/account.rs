use super::Session;
use crate::{protos::spotware_message::*, Error};

impl Session {
    //+------------------------------------------------------------------+
    //|                             Account                              |
    //+------------------------------------------------------------------+
    pub async fn account_list(&self) -> Result<ProtoOaGetAccountListByAccessTokenRes, Error> {
        self.account_list_by_access_token(&self.account.access_token)
            .await
    }

    // Request for getting the list of granted trader's account for the access token.
    // AccessToken下可以有很多账户
    pub async fn account_list_by_access_token(
        &self,
        token: &str,
    ) -> Result<ProtoOaGetAccountListByAccessTokenRes, Error> {
        let req = ProtoOaGetAccountListByAccessTokenReq {
            payload_type: None,
            access_token: token.to_string(),
        };
        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaGetAccountListByAccessTokenRes::from)
    }

    // Request for getting Trader's current open positions and pending orders data.
    pub async fn get_open_position_and_pending_orders(&self) -> Result<ProtoOaReconcileRes, Error> {
        let req = ProtoOaReconcileReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            return_protection_orders: None,
        };
        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaReconcileRes::from)
    }

    // Request for getting data of Trader's Account.
    // **ProtoOaTraderUpdatedEvent** Event that is sent when a Trader is updated on Server side.
    pub async fn get_account_data(&self) -> Result<ProtoOaTraderRes, Error> {
        let req: ProtoOaTraderReq = ProtoOaTraderReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
        };
        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaTraderRes::from)
    }
}
