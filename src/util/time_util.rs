use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

pub fn from_mill_seconds(mill_seconds: i64) -> chrono::DateTime<Utc> {
    chrono::DateTime::<Utc>::from_naive_utc_and_offset(
        NaiveDateTime::from_timestamp_millis(mill_seconds).unwrap(),
        Utc,
    )
}

pub fn timestamp_to_datetime(timestamp: i64) -> DateTime<Utc> {
    let ndt =
        NaiveDateTime::from_timestamp_opt(timestamp / 1000, (timestamp % 1000) as u32).unwrap();
    Utc.from_utc_datetime(&ndt)
}

pub fn timestamp_to_datetime_align_secs(timestamp: i64) -> DateTime<Utc> {
    let ndt = NaiveDateTime::from_timestamp_opt(timestamp / 1000, 0).unwrap();
    Utc.from_utc_datetime(&ndt)
}

#[inline]
pub fn utc_now() -> DateTime<Utc> {
    Utc::now()
}
