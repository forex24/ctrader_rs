use super::Session;
use crate::{protos::spotware_message::*, Error};

#[derive(Debug, Clone)]
pub struct NewOrderParams {
    symbol_id: i64,
    order_type: ProtoOaOrderType,
    trade_side: ProtoOaTradeSide,
    volume: i64,
    limit_price: Option<f64>,
    stop_price: Option<f64>,
    time_in_force: Option<ProtoOaTimeInForce>,
    /// The Unix time in milliseconds of expiration if the order has time in force GTD.
    expiration_timestamp: Option<i64>,
    stop_loss: Option<f64>,
    take_profit: Option<f64>,
    comment: Option<String>,
    base_slippage_price: Option<f64>,
    slippage_in_points: Option<i32>,
    label: Option<String>,
    /// ID of the position linked to the order (e.g. closing order, order that increase volume of a specific position, etc.).
    position_id: Option<i64>,
    client_order_id: Option<String>,
    relative_stop_loss: Option<i64>,
    relative_take_profit: Option<i64>,
    guaranteed_stop_loss: Option<bool>,
    trailing_stop_loss: Option<bool>,
    stop_trigger_method: Option<ProtoOaOrderTriggerMethod>,
}

#[derive(Debug, Clone)]
pub struct ModifyOrderParams {
    volume: Option<i64>,
    limit_price: Option<f64>,
    stop_price: Option<f64>,
    expiration_timestamp: Option<i64>,
    stop_loss: Option<f64>,
    take_profit: Option<f64>,
    slippage_in_points: Option<i32>,
    relative_stop_loss: Option<i64>,
    relative_take_profit: Option<i64>,
    guaranteed_stop_loss: Option<bool>,
    trailing_stop_loss: Option<bool>,
    stop_trigger_method: Option<i32>,
}
impl Session {
    //+------------------------------------------------------------------+
    //|                             Order                                |
    //+------------------------------------------------------------------+

    // Request for sending a new trading order.
    // Allowed only if the accessToken has the "trade" permissions for the trading account.
    pub async fn new_order(&self, params: NewOrderParams) -> Result<ProtoOaExecutionEvent, Error> {
        let req = ProtoOaNewOrderReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            symbol_id: params.symbol_id,
            order_type: params.order_type.into(),
            trade_side: params.trade_side.into(),
            volume: params.volume,
            limit_price: params.limit_price,
            stop_price: params.stop_price,
            time_in_force: params.time_in_force.map(|x| x.into()),
            expiration_timestamp: params.expiration_timestamp,
            stop_loss: params.stop_loss,
            take_profit: params.take_profit,
            comment: params.comment,
            base_slippage_price: params.base_slippage_price,
            slippage_in_points: params.slippage_in_points,
            label: params.label,
            position_id: params.position_id,
            client_order_id: params.client_order_id,
            relative_stop_loss: params.relative_stop_loss,
            relative_take_profit: params.relative_take_profit,
            guaranteed_stop_loss: params.guaranteed_stop_loss,
            trailing_stop_loss: params.trailing_stop_loss,
            stop_trigger_method: params.stop_trigger_method.map(|x| x.into()),
        };

        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaExecutionEvent::from)
    }

    // Request for cancelling existing pending order.
    // Allowed only if the accessToken has "trade" permissions for the trading account.
    pub async fn cancel_order(&self, order_id: i64) -> Result<ProtoOaExecutionEvent, Error> {
        let req = ProtoOaCancelOrderReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            order_id,
        };

        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaExecutionEvent::from)
    }

    // Request for amending the existing pending order.
    // Allowed only if the Access Token has "trade" permissions for the trading account.
    pub async fn modify_order(
        &self,
        order_id: i64,
        params: ModifyOrderParams,
    ) -> Result<ProtoOaExecutionEvent, Error> {
        let req = ProtoOaAmendOrderReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            order_id,
            volume: params.volume,
            limit_price: params.limit_price,
            stop_price: params.stop_price,
            expiration_timestamp: params.expiration_timestamp,
            stop_loss: params.stop_loss,
            take_profit: params.take_profit,
            slippage_in_points: params.slippage_in_points,
            relative_stop_loss: params.relative_stop_loss,
            relative_take_profit: params.relative_take_profit,
            guaranteed_stop_loss: params.guaranteed_stop_loss,
            trailing_stop_loss: params.trailing_stop_loss,
            stop_trigger_method: params.stop_trigger_method,
        };

        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaExecutionEvent::from)
    }

    // Request for getting the margin estimate.
    // Can be used before sending a new order request.
    pub async fn expected_margin(
        &self,
        symbol_id: i64,
        volume: Vec<i64>,
    ) -> Result<ProtoOaExpectedMarginRes, Error> {
        let req = ProtoOaExpectedMarginReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            symbol_id,
            volume,
        };

        self.connection
            .send_request(req.into())
            .await
            .map(ProtoOaExpectedMarginRes::from)
    }
}
