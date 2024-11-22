use tracing::{error, warn};

use super::Session;
use crate::protos::spotware_message::*;

#[derive(Debug, Clone)]
pub enum NotifyEvent {
    None,
    TrailingSlChangedEvent(ProtoOaTrailingSlChangedEvent),
    SymbolChangedEvent(ProtoOaSymbolChangedEvent),
    // TODO: second-largest variant 224bytes
    AccoutDataUpdateEvent(ProtoOaTraderUpdatedEvent),
    // TODO: largest variant 1224bytes
    ExecutionEvent(ProtoOaExecutionEvent),
    SpotEvent(ProtoOaSpotEvent),
    OrderErrorEvent(ProtoOaOrderErrorEvent),
    MarginChangedEvent(ProtoOaMarginChangedEvent),
    ErrorRes(ProtoOaErrorRes),
    AccountsTokenInvalidatedEvent(ProtoOaAccountsTokenInvalidatedEvent),
    ClientDisconnectEvent(ProtoOaClientDisconnectEvent),
    DepthEvent(ProtoOaDepthEvent),
    AccountDisconnectEvent(ProtoOaAccountDisconnectEvent),
    MarginCallUpdateEvent(ProtoOaMarginCallUpdateEvent),
    MarginCallTriggerEvent(ProtoOaMarginCallTriggerEvent),
    ProtoErrorRes(ProtoErrorRes),
}

impl Session {
    /// Event that is sent when the level of the Trailing Stop Loss is changed due to the price level changes.
    //  pub position_id: i64,
    //  pub order_id: i64,
    //  pub stop_price: f64,
    //  pub utc_last_update_timestamp: i64,
    pub fn notify_trailing_sl_changed_event(&mut self, msg: ProtoMessage) -> NotifyEvent {
        let event: ProtoOaTrailingSlChangedEvent = ProtoOaTrailingSlChangedEvent::from(msg);
        NotifyEvent::TrailingSlChangedEvent(event)
    }

    // Event that is sent when the symbol is changed on the Server side.
    // pub symbol_id: Vec<i64>,
    pub fn notify_symbol_changed_event(&mut self, msg: ProtoMessage) -> NotifyEvent {
        let event = ProtoOaSymbolChangedEvent::from(msg);
        NotifyEvent::SymbolChangedEvent(event)
    }

    // Event that is sent when a Trader is updated on Server side.
    // pub trader: ProtoOaTrader,
    pub fn notify_trader_updated_event(&mut self, msg: ProtoMessage) -> NotifyEvent {
        let event = ProtoOaTraderUpdatedEvent::from(msg);
        NotifyEvent::AccoutDataUpdateEvent(event)
    }

    // Event that is sent following the successful order acceptance or execution by the server.
    // Acts as response to the ProtoOANewOrderReq, ProtoOACancelOrderReq, ProtoOAAmendOrderReq, ProtoOAAmendPositionSLTPReq, ProtoOAClosePositionReq requests.
    // Also, the event is sent when a Deposit/Withdrawal took place.
    pub fn notify_execution_event(&mut self, msg: ProtoMessage) -> NotifyEvent {
        let event = ProtoOaExecutionEvent::from(msg);
        NotifyEvent::ExecutionEvent(event)
    }

    // Event that is sent when a new spot event is generated on the server side.
    // Requires subscription on the spot events, see ProtoOASubscribeSpotsReq.
    // First event, received after subscription will contain latest spot prices even if market is closed.
    pub fn notify_spot_event(&mut self, msg: ProtoMessage) -> NotifyEvent {
        let event = ProtoOaSpotEvent::from(msg);
        /*let (quote, candle) = self.aggregator.update(&event);
        if let Some(c) = candle {
            info!("id:{} time:{} Candle: {}", event.symbol_id, Utc::now(), c);
        }
        info!("id:{} quote:{}", event.symbol_id, quote)*/
        NotifyEvent::SpotEvent(event)
    }

    // Event that is sent when errors occur during the order requests.
    pub fn notify_order_error_event(&mut self, msg: ProtoMessage) -> NotifyEvent {
        let event = ProtoOaOrderErrorEvent::from(msg);
        error!("ProtoOaOrderError:{}", event);
        NotifyEvent::OrderErrorEvent(event)
    }

    // Event that is sent when the margin allocated to a specific position is changed.
    pub fn notify_margin_changed_event(&mut self, msg: ProtoMessage) -> NotifyEvent {
        let event = ProtoOaMarginChangedEvent::from(msg);
        NotifyEvent::MarginChangedEvent(event)
    }

    // Generic response when an ERROR occurred.
    pub fn notify_error_res(&mut self, msg: ProtoMessage) {
        let event = ProtoOaErrorRes::from(msg);
        error!("ProtoOaErrorRes:{}", event);
    }

    // Event that is sent when a session to a specific trader's account is terminated by the server
    // but the existing connections with the other trader's accounts are maintained.
    pub async fn notify_accounts_token_invalidated_event(&mut self, msg: ProtoMessage) {
        let event = ProtoOaAccountsTokenInvalidatedEvent::from(msg);
        warn!(
            "Account {:?} terminated by server, reason {:?}",
            event.ctid_trader_account_ids, event.reason
        );
        let _ = self.refresh_token_and_reauth().await;
    }

    // Event that is sent when the connection with the client application is cancelled by the server.
    // All the sessions for the traders' accounts will be terminated.
    pub fn notify_client_disconnect_event(&mut self, msg: ProtoMessage) {
        let event = ProtoOaClientDisconnectEvent::from(msg);
        error!("client_disconnect_event:{:?}", event)
    }

    // Event that is sent when the structure of depth of market is changed.
    // Requires subscription on the depth of markets for the symbol, see ProtoOASubscribeDepthQuotesReq
    pub fn notify_depth_event(&mut self, msg: ProtoMessage) -> NotifyEvent {
        let event = ProtoOaDepthEvent::from(msg);
        NotifyEvent::DepthEvent(event)
    }

    // Event that is sent when the established session for an account is dropped on the server side.
    // A new session must be authorized for the account
    pub fn notify_account_disconnect_event(&mut self, msg: ProtoMessage) {
        let event = ProtoOaAccountDisconnectEvent::from(msg);
        warn!("account {} drop from server", event.ctid_trader_account_id);
    }

    // Event that is sent when a Margin Call threshold configuration is updated.
    pub fn notify_margin_call_update_event(&mut self, msg: ProtoMessage) -> NotifyEvent {
        let event = ProtoOaMarginCallUpdateEvent::from(msg);
        NotifyEvent::MarginCallUpdateEvent(event)
    }

    // Event that is sent when account margin level reaches target marginLevelThreshold.
    // Event is sent no more than once every 10 minutes to avoid spamming.
    pub fn notify_margin_call_trigger_event(&mut self, msg: ProtoMessage) -> NotifyEvent {
        let event = ProtoOaMarginCallTriggerEvent::from(msg);
        NotifyEvent::MarginCallTriggerEvent(event)
    }

    // Event that is sent from Open API proxy and can be used as criteria that connection is healthy when no other messages are sent by cTrader platform.
    // Open API client can send this message when he needs to keep the connection open for a period without other messages longer than 30 seconds
    pub fn notify_proto_heartbeat_event(&mut self, msg: ProtoMessage) {
        let _event = ProtoHeartbeatEvent::from(msg);
    }

    pub fn notify_proto_error_res(&mut self, msg: ProtoMessage) {
        let event = ProtoErrorRes::from(msg);
        error!("ProtoErrorRes:{}", event);
    }

    pub async fn dispatch_event(&mut self, msg: ProtoMessage) -> NotifyEvent {
        match msg.payload_type {
            50 => self.notify_proto_error_res(msg),
            51 => self.notify_proto_heartbeat_event(msg), //unrearchable!
            2142 => self.notify_error_res(msg),           // unrearchable!
            2147 => self.notify_accounts_token_invalidated_event(msg).await,
            2148 => self.notify_client_disconnect_event(msg),

            2107 => return self.notify_trailing_sl_changed_event(msg),
            2120 => return self.notify_symbol_changed_event(msg),
            2123 => return self.notify_trader_updated_event(msg),
            2126 => return self.notify_execution_event(msg),
            2131 => return self.notify_spot_event(msg),
            2132 => return self.notify_order_error_event(msg),
            2141 => return self.notify_margin_changed_event(msg),
            2155 => return self.notify_depth_event(msg),
            2171 => return self.notify_margin_call_update_event(msg),
            2172 => return self.notify_margin_call_trigger_event(msg),
            _ => {
                error!("unknow notify event: {}", msg.payload_type)
            }
        };
        NotifyEvent::None
    }
}
