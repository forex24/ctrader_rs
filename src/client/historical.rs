use super::Session;
use crate::{protos::spotware_message::*, Error};

impl Session {
    //+------------------------------------------------------------------+
    //|                           Historical                             |
    //+------------------------------------------------------------------+

    // Request for getting Trader's closed orders filtered by timestamp
    pub async fn get_historical_order_list(
        &self,
        from_timestamp: i64,
        to_timestamp: i64,
    ) -> Result<ProtoOaOrderListRes, Error> {
        let req = self.make_order_list_req(from_timestamp, to_timestamp);

        self.connection
            .send_historical_request(req.into())
            .await
            .map(ProtoOaOrderListRes::from)
    }

    // Request for getting Trader's deals historical data (execution details).
    pub async fn get_historical_deal_list(
        &self,
        from_timestamp: i64,
        to_timestamp: i64,
        max_rows: Option<i32>,
    ) -> Result<ProtoOaDealListRes, Error> {
        let req = self.make_deal_list_req(from_timestamp, to_timestamp, max_rows);

        self.connection
            .send_historical_request(req.into())
            .await
            .map(ProtoOaDealListRes::from)
    }

    // Request for getting Trader's historical data of deposits and withdrawals.
    pub async fn get_historical_cash_flow_list(
        &self,
        from_timestamp: i64,
        to_timestamp: i64,
    ) -> Result<ProtoOaCashFlowHistoryListRes, Error> {
        let req = self.make_cash_flow_history_list_req(from_timestamp, to_timestamp);

        self.connection
            .send_historical_request(req.into())
            .await
            .map(ProtoOaCashFlowHistoryListRes::from)
    }

    // Request for getting historical trend bars for the symbol.
    pub async fn get_trend_bars(
        &self,
        from_timestamp: i64,
        to_timestamp: i64,
        period: i32,
        symbol_id: i64,
        count: Option<u32>,
    ) -> Result<ProtoOaGetTrendbarsRes, Error> {
        let req = self.make_trend_bars_req(from_timestamp, to_timestamp, period, symbol_id, count);
        self.connection
            .send_historical_request(req.into())
            .await
            .map(ProtoOaGetTrendbarsRes::from)
    }

    // Request for getting historical tick data for the symbol.
    pub async fn get_tick_data(
        &self,
        symbol_id: i64,
        r#type: i32,
        from_timestamp: i64,
        to_timestamp: i64,
    ) -> Result<ProtoOaGetTickDataRes, Error> {
        let req = self.make_tick_data_req(symbol_id, r#type, from_timestamp, to_timestamp);

        self.connection
            .send_historical_request(req.into())
            .await
            .map(ProtoOaGetTickDataRes::from)
    }

    // make_* request

    pub fn make_order_list_req(
        &self,
        from_timestamp: i64,
        to_timestamp: i64,
    ) -> ProtoOaOrderListReq {
        assert!(from_timestamp >= 0);
        assert!(to_timestamp < 2147483646000);
        assert!(to_timestamp - from_timestamp <= 604800000); // 1 week
        ProtoOaOrderListReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            from_timestamp,
            to_timestamp,
        }
    }

    pub fn make_deal_list_req(
        &self,
        from_timestamp: i64,
        to_timestamp: i64,
        max_rows: Option<i32>,
    ) -> ProtoOaDealListReq {
        assert!(from_timestamp >= 0);
        assert!(to_timestamp < 2147483646000);
        assert!(to_timestamp - from_timestamp <= 604800000); // 1 week
        ProtoOaDealListReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            from_timestamp,
            to_timestamp,
            max_rows,
        }
    }

    pub fn make_cash_flow_history_list_req(
        &self,
        from_timestamp: i64,
        to_timestamp: i64,
    ) -> ProtoOaCashFlowHistoryListReq {
        assert!(from_timestamp >= 0);
        assert!(to_timestamp < 2147483646000);
        assert!(to_timestamp - from_timestamp <= 604800000); // 1 week
        ProtoOaCashFlowHistoryListReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            from_timestamp,
            to_timestamp,
        }
    }

    pub fn make_trend_bars_req(
        &self,
        from_timestamp: i64,
        to_timestamp: i64,
        period: i32,
        symbol_id: i64,
        count: Option<u32>,
    ) -> ProtoOaGetTrendbarsReq {
        // from_timestamp:The Unix time in milliseconds from which the search starts. Must be bigger or equal to zero (1st Jan 1970).
        // to_timestamp:The Unix time in milliseconds of finishing the search. Smaller or equal to 2147483646000 (19th Jan 2038).
        // Validation: toTimestamp - fromTimestamp <= X, where X depends on series period: M1, M2, M3, M4, M5: 3024000000 (5 weeks);
        // M10, M15, M30, H1: 21168000000 (35 weeks),
        // H4, H12, D1: 31622400000 (1 year); W1, MN1: 158112000000 (5 years).
        assert!(from_timestamp >= 0);
        assert!(to_timestamp < 2147483646000);
        assert!(
            (period <= 5 && to_timestamp - from_timestamp <= 302400000)
                || ((6..=9).contains(&period) && to_timestamp - from_timestamp <= 21168000000)
                || ((10..=12).contains(&period) && to_timestamp - from_timestamp <= 31622400000)
                || ((13..=14).contains(&period) && to_timestamp - from_timestamp <= 158112000000)
        );
        ProtoOaGetTrendbarsReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            from_timestamp,
            to_timestamp,
            period,
            symbol_id,
            count,
        }
    }

    pub fn make_tick_data_req(
        &self,
        symbol_id: i64,
        r#type: i32,
        from_timestamp: i64,
        to_timestamp: i64,
    ) -> ProtoOaGetTickDataReq {
        // The Unix time in milliseconds of starting the search. Must be bigger or equal to zero (1st Jan 1970). Validation: toTimestamp - fromTimestamp <= 604800000 (1 week).
        // The Unix time in milliseconds of finishing the search. <= 2147483646000 (19th Jan 2038).
        assert!(from_timestamp >= 0);
        assert!(to_timestamp < 2147483646000);
        assert!(to_timestamp - from_timestamp <= 604800000); // 1 week
        ProtoOaGetTickDataReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            symbol_id,
            r#type,
            from_timestamp,
            to_timestamp,
        }
    }
}
