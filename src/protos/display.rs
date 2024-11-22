use crate::protos::spotware_message::*;

impl std::fmt::Display for ProtoErrorRes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "code: {}, description: {:?}, maintenance_end_timestamp: {:?}",
            self.error_code, self.description, self.maintenance_end_timestamp
        )
    }
}

impl std::fmt::Display for ProtoOaErrorRes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "account: {:?} code: {}, description: {:?}",
            self.ctid_trader_account_id, self.error_code, self.description
        )
    }
}

impl std::fmt::Display for ProtoOaTrader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "account: {:?} balance: {}, balance_version: {:?}",
            self.ctid_trader_account_id,
            self.balance,
            self.balance_version()
        )
    }
}

impl std::fmt::Display for ProtoOaTrendbar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let low = self.low.unwrap();
        let open = self.delta_open.unwrap() as i64 + low;
        let high = self.delta_high.unwrap() as i64 + low;
        let close = self.delta_close.unwrap_or(0) as i64 + low;
        let vol = self.volume;
        let time_in_minutes = self.utc_timestamp_in_minutes.unwrap();
        write!(
            f,
            "T:{} O:{} H:{} L:{} C:{} V:{}",
            time_in_minutes, open, high, low, close, vol
        )
    }
}

impl std::fmt::Display for ProtoOaOrderErrorEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "account: {:?} error_code: {}, order_id: {:?} position_id: {:?} description: {:?}",
            self.ctid_trader_account_id,
            self.error_code,
            self.order_id,
            self.position_id,
            self.description
        )
    }
}
