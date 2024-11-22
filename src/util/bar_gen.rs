use tracing::error;

use crate::protos::spotware_message::ProtoOaSpotEvent;
use crate::util::time_util;

#[derive(Default, Debug, Clone, Copy)]
/// Defines a Candle
pub struct Candle {
    /// latest timestamp of last received trade
    pub timestamp: i64,
    /// open price of candle
    pub open: f64,
    /// high price of candle
    pub high: f64,
    /// low price of candle
    pub low: f64,
    /// close price of candle
    pub close: f64,
    /// volume
    pub vol: u64,
    /// number of taker trades observed in candle
    pub num_asks: i32,
    pub num_bids: i32,
}

impl std::fmt::Display for Candle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(ts: {:?}, o: {:.5}, h: {:.5}, l: {:.5}, c: {:.5}, v: {} ac: {} bc: {})",
            time_util::timestamp_to_datetime_align_secs(self.timestamp),
            self.open,
            self.high,
            self.low,
            self.close,
            self.vol,
            self.num_asks,
            self.num_bids
        )
    }
}

#[derive(Default, Debug, Clone, Copy)]
/// Defines a Candle
pub struct Quote {
    /// latest timestamp of last received trade
    pub timestamp: i64,
    pub ask: f64,
    pub bid: f64,
}

impl std::fmt::Display for Quote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(ts: {:?}, ask: {:.5}, bid: {:.5})",
            time_util::timestamp_to_datetime(self.timestamp),
            self.ask,
            self.bid,
        )
    }
}

#[derive(Debug, Clone)]
/// Struct used for aggregating trades by time in an online (streaming) manner
pub struct BarGenerator {
    period: u32, // in minutes
    timestamp: u32,
    open: f64,
    high: f64,
    low: f64,
    vol: u64,
    num_bids: i32,
    num_asks: i32,
    last_ask: f64,
    last_bid: f64,
}

impl BarGenerator {
    pub fn new(period: u32) -> Self {
        Self {
            period,
            timestamp: 0,
            open: 0.0,
            high: 0.0,
            low: 0.0,
            vol: 0,
            num_asks: 0,
            num_bids: 0,
            last_ask: 0.0,
            last_bid: 0.0,
        }
    }

    fn frozen_candle(&mut self) -> Option<Candle> {
        let candle = if self.timestamp == 0 {
            None
        } else {
            Some(Candle {
                timestamp: (self.timestamp as i64) * 60 * 1000,
                open: self.open,
                high: self.high,
                low: self.low,
                close: self.last_bid,
                vol: self.vol,
                num_asks: self.num_asks,
                num_bids: self.num_bids,
            })
        };
        self.num_bids = 0;
        self.num_asks = 0;
        self.vol = 0;
        candle
    }

    pub fn map_api_period_to_minutes(api_period: u32) -> u32 {
        match api_period {
            1..=5 => api_period,
            6 => 10,
            7 => 15,
            8 => 30,
            9 => 60,
            10 => 4 * 60,
            11 => 12 * 60,
            12 => 24 * 60,
            13 => 7 * 24 * 60,
            _ => {
                error!("api period error:{}", api_period);
                0
            }
        }
    }

    pub fn update(&mut self, event: &ProtoOaSpotEvent) -> (Quote, Option<Candle>) {
        // update ask/bid
        let ask = event
            .ask
            .map(|x| (x as f64) / 100_000.0)
            .unwrap_or(self.last_ask);
        let bid = event
            .bid
            .map(|x| (x as f64) / 100_000.0)
            .unwrap_or(self.last_bid);

        if event.ask.is_some() {
            self.num_asks += 1;
        }
        if event.bid.is_some() {
            self.num_bids += 1;
        }
        self.last_ask = ask;
        self.last_bid = bid;

        let event_time = if !event.trendbar.is_empty() {
            event.trendbar[0].utc_timestamp_in_minutes.unwrap()
        } else {
            (event.timestamp.unwrap() / 1000 / 60 / self.period as i64) as u32
        };

        let result = if event_time != self.timestamp {
            self.frozen_candle()
        } else {
            None
        };

        self.timestamp = event_time;

        if !event.trendbar.is_empty() {
            // update OHLC
            debug_assert!(
                Self::map_api_period_to_minutes(event.trendbar[0].period.unwrap() as u32)
                    == self.period
            );

            let _low = event.trendbar[0].low.unwrap();
            let _open = event.trendbar[0].delta_open.unwrap_or(0) as i64 + _low;
            let _high = event.trendbar[0].delta_high.unwrap_or(0) as i64 + _low;
            let low = _low as f64 / 100_000.0;
            let open = _open as f64 / 100_000.0;
            let high = _high as f64 / 100_000.0;
            let vol = event.trendbar[0].volume as u64;

            self.open = open;
            self.high = high;
            self.low = low;
            self.vol += vol;
        }
        let quote = Quote {
            timestamp: event.timestamp.unwrap(),
            ask: self.last_ask,
            bid: self.last_bid,
        };
        (quote, result)
    }
}
