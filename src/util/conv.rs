use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use crate::protos::spotware_message::*;

#[derive(Debug, Clone)]
pub struct TrendBar {
    period:i32,
    time:DateTime<Utc>,
    open:f64,
    high:f64,
    low:f64,
    close:f64,
    vol: u64
}

pub fn convert_trendbar(bar: ProtoOaTrendbar) -> TrendBar {
    let timestamp = bar.utc_timestamp_in_minutes.unwrap() as i64 * 60;
    let _low = bar.low.unwrap();
    let _open = bar.delta_open.unwrap_or(0) as i64 + _low;
    let _high = bar.delta_high.unwrap_or(0) as i64 + _low;
    let _close = bar.delta_close.unwrap_or(0) as i64 + _low;
    let low = _low as f64 / 100_000.0;
    let open = _open as f64 / 100_000.0;
    let high = _high as f64 / 100_000.0;
    let close = _close as f64 / 100_000.0;
    let vol = bar.volume as u64;

    TrendBar {
        period: bar.period.unwrap(),
        time:Utc
        .from_utc_datetime(&NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap()),
        open,
        high,
        low,
        close,
        vol
    }
}
