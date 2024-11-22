use super::Session;
use crate::{protos::spotware_message::*, Error};

impl Session {
    // Request for amending StopLoss and TakeProfit of existing position.
    // Allowed only if the accessToken has "trade" permissions for the trading account.
    pub async fn modify_position_sltp(
        &self,
        position_id: i64,
        stop_loss: Option<f64>,
        take_profit: Option<f64>,
        guaranteed_stop_loss: Option<bool>,
        trailing_stop_loss: Option<bool>,
        stop_loss_trigger_method: Option<i32>,
    ) -> Result<ProtoOaExecutionEvent, Error> {
        let req = ProtoOaAmendPositionSltpReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            position_id,
            stop_loss,
            take_profit,
            guaranteed_stop_loss,
            trailing_stop_loss,
            stop_loss_trigger_method,
        };
        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaExecutionEvent::from)
    }

    // Request for closing or partially closing of an existing position.
    // Allowed only if the accessToken has "trade" permissions for the trading account.
    pub async fn close_position(
        &self,
        position_id: i64,
        volume: i64,
    ) -> Result<ProtoOaExecutionEvent, Error> {
        let req = ProtoOaClosePositionReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            position_id,
            volume,
        };
        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaExecutionEvent::from)
    }

    pub async fn order_list_by_position_id(
        &self,
        position_id: i64,
        from_timestamp: i64,
        to_timestamp: i64,
    ) -> Result<ProtoOaOrderListByPositionIdRes, Error> {
        let req = ProtoOaOrderListByPositionIdReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            position_id,
            from_timestamp,
            to_timestamp,
        };
        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaOrderListByPositionIdRes::from)
    }

    // Request for retrieving the deals related to a position.
    pub async fn deal_list_position_id(
        &self,
        position_id: i64,
        from_timestamp: i64,
        to_timestamp: i64,
    ) -> Result<ProtoOaDealListByPositionIdRes, Error> {
        assert!(from_timestamp >= 0);
        assert!(to_timestamp < 2147483646000);
        assert!(to_timestamp - from_timestamp <= 604800000); // 1 week
        let req = ProtoOaDealListByPositionIdReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            position_id,
            from_timestamp,
            to_timestamp,
        };

        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaDealListByPositionIdRes::from)
    }
}
