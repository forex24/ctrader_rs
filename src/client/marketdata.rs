use tracing::error;

use super::Session;
use crate::{protos::spotware_message::*, Error};

impl Session {
    //+------------------------------------------------------------------+
    //|                              Quotes                              |
    //+------------------------------------------------------------------+

    // Request for subscribing on spot events of the specified symbol.
    pub async fn subscribe_spot(&mut self, symbol_ids: Vec<i64>) -> Result<(), Error> {
        let req = ProtoOaSubscribeSpotsReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            symbol_id: symbol_ids.clone(),
            subscribe_to_spot_timestamp: Some(true), // If TRUE you will also receive the timestamp in ProtoOASpotEvent.
        };

        let m = self
            .connection
            .send_request(req.into())
            .await
            .map(ProtoOaSubscribeSpotsRes::from);

        match m {
            Err(e) => return Err(e),
            Ok(m) => {
                if m.ctid_trader_account_id != self.account.account_id {
                    error!(
                        "subscribe_spot account id {} != {}",
                        m.ctid_trader_account_id, self.account.account_id
                    );
                }
                self.update_subscribe_spot(&symbol_ids, true);
            }
        }

        Ok(())
    }

    // Request for unsubscribing from the spot events of the specified symbol.
    pub async fn unsubscribe_spot(&mut self, symbol_ids: Vec<i64>) -> Result<(), Error> {
        let req = ProtoOaUnsubscribeSpotsReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            symbol_id: symbol_ids.clone(),
        };

        let m = self
            .connection
            .send_request(req.into())
            .await
            .map(ProtoOaUnsubscribeSpotsRes::from);

        match m {
            Err(e) => return Err(e),
            Ok(m) => {
                if m.ctid_trader_account_id != self.account.account_id {
                    error!(
                        "unsubscribe_spot account id {} != {}",
                        m.ctid_trader_account_id, self.account.account_id
                    );
                }

                self.update_subscribe_spot(&symbol_ids, false);
            }
        }

        Ok(())
    }

    // Request for subscribing for live trend bars.
    // Requires subscription on the spot events, see ProtoOASubscribeSpotsReq.
    pub async fn subscribe_live_bar(&mut self, period: i32, symbol_id: i64) -> Result<(), Error> {
        let req = ProtoOaSubscribeLiveTrendbarReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            period,
            symbol_id,
        };

        let m = self
            .connection
            .send_request(req.into())
            .await
            .map(ProtoOaSubscribeLiveTrendbarRes::from);

        match m {
            Err(e) => return Err(e),
            Ok(m) => {
                if m.ctid_trader_account_id != self.account.account_id {
                    error!(
                        "subscribe_live_bar account id {} != {}",
                        m.ctid_trader_account_id, self.account.account_id
                    );
                }

                self.update_subscribe_bar(period, symbol_id, true);
            }
        }

        Ok(())
    }

    // Request for unsubscribing from the live trend bars.
    pub async fn unsubscribe_live_bar(&mut self, period: i32, symbol_id: i64) -> Result<(), Error> {
        let req = ProtoOaUnsubscribeLiveTrendbarReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            period,
            symbol_id,
        };

        let m = self
            .connection
            .send_request(req.into())
            .await
            .map(ProtoOaUnsubscribeLiveTrendbarRes::from);

        match m {
            Err(e) => return Err(e),
            Ok(m) => {
                if m.ctid_trader_account_id != self.account.account_id {
                    error!(
                        "unsubscribe_live_bar account id {} != {}",
                        m.ctid_trader_account_id, self.account.account_id
                    );
                }

                self.update_subscribe_bar(period, symbol_id, false);
            }
        }

        Ok(())
    }

    // Request for subscribing on depth of market of the specified symbol.
    pub async fn subscribe_depth_quotes(&mut self, symbol_ids: Vec<i64>) -> Result<(), Error> {
        let req = ProtoOaSubscribeDepthQuotesReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            symbol_id: symbol_ids.clone(),
        };

        let m = self
            .connection
            .send_request(req.into())
            .await
            .map(ProtoOaSubscribeDepthQuotesRes::from);

        match m {
            Err(e) => return Err(e),
            Ok(m) => {
                if m.ctid_trader_account_id != self.account.account_id {
                    error!(
                        "subscribe_depth_quotes_req account id {} != {}",
                        m.ctid_trader_account_id, self.account.account_id
                    );
                }
                self.update_subscribe_depth(&symbol_ids, true);
            }
        }

        Ok(())
    }

    // Request for unsubscribing from the depth of market of the specified symbol.
    pub async fn unsubscribe_depth_quotes(&mut self, symbol_ids: Vec<i64>) -> Result<(), Error> {
        let req = ProtoOaUnsubscribeDepthQuotesReq {
            payload_type: None,
            ctid_trader_account_id: self.account.account_id,
            symbol_id: symbol_ids.clone(),
        };

        let m = self
            .connection
            .send_request(req.into())
            .await
            .map(ProtoOaUnsubscribeDepthQuotesRes::from);
        match m {
            Err(e) => return Err(e),
            Ok(m) => {
                if m.ctid_trader_account_id != self.account.account_id {
                    error!(
                        "unsubscribe_depth_quotes_req account id {} != {}",
                        m.ctid_trader_account_id, self.account.account_id
                    );
                }
                self.update_subscribe_depth(&symbol_ids, false);
            }
        }

        Ok(())
    }

    // subscribe
    pub async fn resume_subscribe(&mut self) -> Result<(), Error> {
        if !self.subscribed_spots.is_empty() {
            self.subscribe_spot(self.subscribed_spots.clone()).await?;
        }

        if !self.subscribed_depths.is_empty() {
            self.subscribe_depth_quotes(self.subscribed_depths.clone())
                .await?;
        }

        if !self.subscribed_bars.is_empty() {
            for (period, symbol_id) in self.subscribed_bars.clone() {
                self.subscribe_live_bar(period, symbol_id).await?;
            }
        }

        Ok(())
    }

    fn update_subscribe_spot(&mut self, symbol_ids: &Vec<i64>, add_or_delete: bool) {
        if add_or_delete {
            // add
            self.subscribed_spots.extend(symbol_ids.iter());
            self.subscribed_spots.dedup();
        } else {
            // delete
            for id in symbol_ids {
                let index = self
                    .subscribed_spots
                    .iter()
                    .position(|x| *x == *id)
                    .unwrap();
                self.subscribed_spots.remove(index);
            }
        }
    }

    fn update_subscribe_bar(&mut self, period: i32, symbol_id: i64, add_or_delete: bool) {
        if add_or_delete {
            // add
            self.subscribed_bars.push((period, symbol_id));
            self.subscribed_bars.dedup();
        } else {
            //delete
            let index = self
                .subscribed_bars
                .iter()
                .position(|x| x.0 == period && x.1 == symbol_id)
                .unwrap();
            self.subscribed_bars.remove(index);
        }
    }

    fn update_subscribe_depth(&mut self, symbol_ids: &Vec<i64>, add_or_delete: bool) {
        if add_or_delete {
            // add
            self.subscribed_depths.extend(symbol_ids.iter());
            self.subscribed_depths.dedup();
        } else {
            //delete
            for id in symbol_ids {
                let index = self
                    .subscribed_depths
                    .iter()
                    .position(|x| *x == *id)
                    .unwrap();
                self.subscribed_depths.remove(index);
            }
        }
    }
}

// session_close 是上一个交易日的收盘价
// timestamp: Milliseconds
//
// Reference
// ProtoOaSpotEvent {
//      payload_type: None,
//      ctid_trader_account_id: 24419665,
//      symbol_id: 7,
//      bid: None,
//      ask: Some(18181600),
//      trendbar: [],
//      session_close: None,
//      timestamp: Some(1697679042479)
//  }
//
// ProtoOaSpotEvent {
//      payload_type: None,
//      ctid_trader_account_id: 24419665,
//      symbol_id: 1,
//      bid: Some(105380),
//      ask: Some(105380),
//      trendbar: [ProtoOaTrendbar
//              {   volume: 50,
//                  period: Some(M1),
//                  low: Some(105374),
//                  delta_open: Some(8),
//                  delta_close: None,
//                  delta_high: Some(10),
//                  utc_timestamp_in_minutes: Some(28294650)
//              }],
//      session_close: None,
//      timestamp: Some(1697679042852)
//      }
// 第一个接收到消息
// ProtoOaSpotEvent {
//      payload_type: None,
//      ctid_trader_account_id: 24419665,
//      symbol_id: 1,
//      bid: Some(105376),
//      ask: Some(105376),
//      trendbar: [],
//      session_close: Some(105364),
//      timestamp: Some(1697679031462)
//  }
