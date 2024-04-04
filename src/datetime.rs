#![allow(unused_imports)]
use std::ops::DerefMut;
use chrono::{DateTime, FixedOffset};
use rhai::plugin::*;
use rhai::{Locked, Shared};

#[inline(always)]
fn borrow_mut(datetime: &Shared<Locked<DateTime<FixedOffset>>>) -> impl DerefMut<Target = DateTime<FixedOffset>> + '_ {
    #[cfg(not(feature = "sync"))] return datetime.borrow_mut();

    #[cfg(feature = "sync")] return datetime.write().unwrap();
}

#[export_module]
pub mod datetime_module {

    use chrono::{DateTime, Local, NaiveDateTime};
    use chrono::FixedOffset;
    use chrono::Utc;
    use rhai::{EvalAltResult, Position, Shared, Locked};

    pub type DateTimeFixed = Shared<Locked<DateTime<FixedOffset>>>;

    #[rhai_fn(return_raw)]
    pub fn datetime_now() -> Result<DateTimeFixed, Box<EvalAltResult>> {
        Ok(Shared::new(Locked::new(Utc::now().fixed_offset())))
    }

    #[rhai_fn(return_raw)]
    pub fn datetime_local() -> Result<DateTimeFixed, Box<EvalAltResult>> {
        Ok(Shared::new(Locked::new(Local::now().fixed_offset())))
    }

    #[rhai_fn(return_raw)]
    pub fn datetime_unix(secs: i64) -> Result<DateTimeFixed, Box<EvalAltResult>> {
        DateTime::from_timestamp(secs, 0)
            .ok_or::<Box<EvalAltResult>>(Box::new(EvalAltResult::ErrorDataTooLarge(
                "Timestamp out of range".to_string(),
                Position::NONE,
            )))
            .map(|dt| Shared::new(Locked::new(dt.fixed_offset())))
    }

    #[rhai_fn(return_raw)]
    pub fn datetime_millis(millis: i64) -> Result<DateTimeFixed, Box<EvalAltResult>> {
        DateTime::from_timestamp_millis(millis)
            .ok_or::<Box<EvalAltResult>>(Box::new(EvalAltResult::ErrorDataTooLarge(
                "Timestamp out of range".to_string(),
                Position::NONE,
            )))
            .map(|dt| Shared::new(Locked::new(dt.fixed_offset())))
    }

    #[rhai_fn(return_raw)]
    pub fn datetime_micros(micros: i64) -> Result<DateTimeFixed, Box<EvalAltResult>> {
        DateTime::from_timestamp_micros(micros)
            .ok_or::<Box<EvalAltResult>>(Box::new(EvalAltResult::ErrorDataTooLarge(
                "Timestamp out of range".to_string(),
                Position::NONE,
            )))
            .map(|dt| Shared::new(Locked::new(dt.fixed_offset())))
    }

    #[rhai_fn(return_raw)]
    pub fn datetime_nanos(nanos: i64) -> Result<DateTimeFixed, Box<EvalAltResult>> {
        Ok(Shared::new(Locked::new(
            DateTime::from_timestamp_nanos(nanos).fixed_offset(),
        )))
    }

    #[rhai_fn(return_raw)]
    pub fn datetime_rfc2822(timestamp: &str) -> Result<DateTimeFixed, Box<EvalAltResult>> {
        DateTime::parse_from_rfc2822(timestamp)
            .map_err(|e| {
                Box::new(EvalAltResult::ErrorSystem(
                    "Failed to parse RFC2822 timestamp".to_string(),
                    Box::new(e),
                ))
            })
            .map(|dt| Shared::new(Locked::new(dt)))
    }

    #[rhai_fn(return_raw)]
    pub fn datetime_rfc3339(timestamp: &str) -> Result<DateTimeFixed, Box<EvalAltResult>> {
        DateTime::parse_from_rfc3339(timestamp)
            .map_err(|e| {
                Box::new(EvalAltResult::ErrorSystem(
                    "Failed to parse RFC3339 timestamp".to_string(),
                    Box::new(e),
                ))
            })
            .map(|dt| Shared::new(Locked::new(dt)))
    }

    #[rhai_fn(return_raw)]
    pub fn datetime_parse(timestamp: &str, format: &str) -> Result<DateTimeFixed, Box<EvalAltResult>> {
        NaiveDateTime::parse_from_str(timestamp, format)
            .map_err(|e| {
                Box::new(EvalAltResult::ErrorSystem(
                    format!(
                        "Failed to parse timestamp {} using format {}",
                        timestamp,
                        format
                    ),
                    Box::new(e),
                ))
            })
            .map(|dt| Shared::new(Locked::new(dt.and_utc().fixed_offset())))
    }

    

}
