use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use tracing::info;

use crate::{
    client::Session, protos::spotware_message::ProtoOaGetTrendbarsRes,
    util::time_util::from_mill_seconds, Error,
};

pub struct Kline {
    /// latest timestamp of last received trade
    pub timestamp: DateTime<Utc>,
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
}

impl std::fmt::Display for Kline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(ts: {}, o: {:.5}, h: {:.5}, l: {:.5}, c: {:.5}, V: {})",
            self.timestamp, self.open, self.high, self.low, self.close, self.vol,
        )
    }
}

//const MILLIS_SECS_PER_DAY: i64 = 86400_000;

pub async fn download_asset(
    client: &Session,
    symbol: &str,
    period: i32,
    start: &DateTime<Utc>,
    end: &DateTime<Utc>,
) -> Result<Vec<Kline>, Error> {
    let symbol_id = client.store.get_id_by_name(symbol);
    if symbol_id.is_none() {
        return Err(Error::String(format!(
            "Not Found SymbolId for Symbol:{}",
            symbol
        )));
    }
    let symbol_id = symbol_id.unwrap();
    // Validation: toTimestamp - fromTimestamp <= X, where X depends on series period:
    // M1, M2, M3, M4, M5: 302_400_000 (5 weeks);
    // M10, M15, M30, H1: 21168000000 (35 weeks),
    // H4, H12, D1: 31_622_400_000 (1 year);
    // W1, MN1: 158112000000 (5 years).

    // round datetime to day
    //let start = start_date.date_naive().and_hms_opt(0, 0, 0).unwrap();
    //let end = end_date.date_naive().and_hms_opt(23, 59, 59).unwrap();

    let mut from_timestamp = start.timestamp_millis();
    let to_timestamp = end.timestamp_millis();

    //let days = (to_timestamp - from_timestamp) / MILLIS_SECS_PER_DAY;
    info!(
        "Try to download {} : from:{} -> to:{} ",
        symbol,
        from_mill_seconds(from_timestamp),
        from_mill_seconds(to_timestamp)
    );
    let incrment: i64 = match period {
        1 | 2 | 3 | 4 | 5 => 302_400_000, // M1 M2 M3 M4 M5 5weeks
        6 | 7 | 8 | 9 => 21_168_000_000,  // M10 M15 M30 H1 35weeks
        10 | 11 | 12 => 31_622_400_000,   // H4 H12 D1      1years
        13 | 14 => 158_112_000_000,       // W1 Mn1         5years
        _ => return Err(Error::PeriodParamError(period)),
    };
    /*match period {
        ProtoOaTrendbarPeriod::M1
        | ProtoOaTrendbarPeriod::M2
        | ProtoOaTrendbarPeriod::M3
        | ProtoOaTrendbarPeriod::M4
        | ProtoOaTrendbarPeriod::M5 => 302_400_000 as i64,
        ProtoOaTrendbarPeriod::M10
        | ProtoOaTrendbarPeriod::M15
        | ProtoOaTrendbarPeriod::M30
        | ProtoOaTrendbarPeriod::H1 => 21168000000 as i64,
        ProtoOaTrendbarPeriod::H4 | ProtoOaTrendbarPeriod::H12 | ProtoOaTrendbarPeriod::D1 => {
            31_622_400_000 as i64,
        },
        ProtoOaTrendbarPeriod::W1 | ProtoOaTrendbarPeriod::Mn1 => 158112000000 as i64,
    };*/

    let mut candles = Vec::new();
    while from_timestamp < to_timestamp {
        let mut to = from_timestamp + incrment;
        if to > to_timestamp {
            to = to_timestamp
        }

        let event: ProtoOaGetTrendbarsRes = client
            .get_trend_bars(
                from_timestamp,
                to,
                period, //ProtoOaTrendbarPeriod::M1 as i32,
                symbol_id,
                None,
            )
            .await?;
        save_bar(event, &mut candles, to);

        from_timestamp += incrment;
    }

    Ok(candles)
}

fn save_bar(event: ProtoOaGetTrendbarsRes, candles: &mut Vec<Kline>, last_to_timestamp: i64) {
    for bar in &event.trendbar {
        let timestamp_as_sec = bar.utc_timestamp_in_minutes.unwrap() as i64 * 60;
        if timestamp_as_sec * 1000 >= last_to_timestamp {
            // [from, to) 所以 时间戳要小于 last_to_timestamp
            break;
        }
        let _low = bar.low.unwrap();
        let _open = bar.delta_open.unwrap_or(0) as i64 + _low;
        let _high = bar.delta_high.unwrap_or(0) as i64 + _low;
        let _close = bar.delta_close.unwrap_or(0) as i64 + _low;
        let low = _low as f64 / 100_000.0;
        let open = _open as f64 / 100_000.0;
        let high = _high as f64 / 100_000.0;
        let close = _close as f64 / 100_000.0;
        let vol = bar.volume as u64;

        let kline = Kline {
            timestamp: Utc.from_utc_datetime(
                &NaiveDateTime::from_timestamp_opt(timestamp_as_sec, 0).unwrap(),
            ),
            open,
            high,
            low,
            close,
            vol,
        };
        if candles.is_empty() || kline.timestamp != candles.last().unwrap().timestamp {
            candles.push(kline);
        }
    }
}
