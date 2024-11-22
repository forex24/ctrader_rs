use super::Session;
use crate::{protos::spotware_message::*, Error};

impl Session {
    //+------------------------------------------------------------------+
    //|                              Margin                              |
    //+------------------------------------------------------------------+
    // Request for a list of existing margin call thresholds configured for a user.
    pub async fn margin_call_list(&self) -> Result<ProtoOaMarginCallListRes, Error> {
        let req = ProtoOaMarginCallListReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
        };

        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaMarginCallListRes::from)
    }

    // Request to modify marginLevelThreshold of specified marginCallType for ctidTraderAccountId.
    pub async fn margin_call_update(
        &self,
        margin_call: ProtoOaMarginCall,
    ) -> Result<ProtoOaMarginCallUpdateRes, Error> {
        let req = ProtoOaMarginCallUpdateReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            margin_call,
        };

        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaMarginCallUpdateRes::from)
    }

    // Request for getting a dynamic leverage entity referenced in ProtoOASymbol.
    pub async fn get_dynamic_leverage_by_id(
        &self,
        leverage_id: i64,
    ) -> Result<ProtoOaGetDynamicLeverageByIdRes, Error> {
        let req = ProtoOaGetDynamicLeverageByIdReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            leverage_id,
        };

        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaGetDynamicLeverageByIdRes::from)
    }
}
