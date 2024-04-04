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

    use std::str::FromStr;

    use chrono::{DateTime, Local, NaiveDateTime};
    use chrono::FixedOffset;
    use chrono::Utc;
    use chrono::Locale;

    use rhai::{EvalAltResult, Position, Shared, Locked};

    /// Alias type to bridge rhai and chrono
    pub type DateTimeFixed = Shared<Locked<DateTime<FixedOffset>>>;

    /// Construct DateTime with current UTC time
    #[rhai_fn(return_raw, name = "datetime_utc", name = "datetime_now")]
    pub fn datetime_utc() -> Result<DateTimeFixed, Box<EvalAltResult>> {
        Ok(Shared::new(Locked::new(Utc::now().fixed_offset())))
    }

    /// Construct DateTime with current local time
    #[rhai_fn(return_raw)]
    pub fn datetime_local() -> Result<DateTimeFixed, Box<EvalAltResult>> {
        Ok(Shared::new(Locked::new(Local::now().fixed_offset())))
    }

    /// Construct DateTime with UNIX timestamp
    #[rhai_fn(return_raw)]
    pub fn datetime_unix(secs: i64) -> Result<DateTimeFixed, Box<EvalAltResult>> {
        DateTime::from_timestamp(secs, 0)
            .ok_or::<Box<EvalAltResult>>(
                Box::<EvalAltResult>::from("Timestamp out of range".to_string())
            )
            .map(|dt| Shared::new(Locked::new(dt.fixed_offset())))
    }

    /// Construct DateTime with UNIX timestamp in milliseconds
    #[rhai_fn(return_raw)]
    pub fn datetime_millis(millis: i64) -> Result<DateTimeFixed, Box<EvalAltResult>> {
        DateTime::from_timestamp_millis(millis)
            .ok_or::<Box<EvalAltResult>>(
                Box::<EvalAltResult>::from("Timestamp out of range".to_string())
            )
            .map(|dt| Shared::new(Locked::new(dt.fixed_offset())))
    }

    /// Construct DateTime with UNIX timestamp in microseconds
    #[rhai_fn(return_raw)]
    pub fn datetime_micros(micros: i64) -> Result<DateTimeFixed, Box<EvalAltResult>> {
        DateTime::from_timestamp_micros(micros)
            .ok_or::<Box<EvalAltResult>>(
                Box::<EvalAltResult>::from("Timestamp out of range".to_string())
            )
            .map(|dt| Shared::new(Locked::new(dt.fixed_offset())))
    }

    /// Construct DateTime with UNIX timestamp in nanoseconds
    #[rhai_fn(return_raw)]
    pub fn datetime_nanos(nanos: i64) -> Result<DateTimeFixed, Box<EvalAltResult>> {
        Ok(Shared::new(Locked::new(
            DateTime::from_timestamp_nanos(nanos).fixed_offset(),
        )))
    }

    /// Construct DateTime with valid RFC2822 timestamp
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

    /// Construct DateTime with valid RFC3339 timestamp
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

    /// Construct DateTime from custom timestamp and format
    /// See the [format::strftime](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) module for supported format sequences.
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

    /// Output RFC3339 string
    #[rhai_fn(global, name = "to_string", name = "to_rfc3339", pure)]
    pub fn to_string(dt: &mut DateTimeFixed) -> ImmutableString {
        borrow_mut(dt).to_rfc3339().into()
    }

    /// Output RFC2822 string
    #[rhai_fn(global, name = "to_rfc2822", pure)]
    pub fn to_rfc2822(dt: &mut DateTimeFixed) -> ImmutableString {
        borrow_mut(dt).to_rfc2822().into()
    }

    /// Output UNIX timestamp i64
    #[rhai_fn(global, name = "timestamp", pure)]
    pub fn timestamp(dt: &mut DateTimeFixed) -> i64 {
        borrow_mut(dt).timestamp()
    }

    /// Output UNIX timestamp in milliseconds
    #[rhai_fn(global, name = "timestamp_millis", pure)]
    pub fn timestamp_millis(dt: &mut DateTimeFixed) -> i64 {
        borrow_mut(dt).timestamp_millis()
    }

    /// Output UNIX timestamp in microseconds
    #[rhai_fn(global, name = "timestamp_micros", pure)]
    pub fn timestamp_micros(dt: &mut DateTimeFixed) -> i64 {
        borrow_mut(dt).timestamp_micros()
    }

    /// Output UNIX timestamp in nanoseconds
    #[rhai_fn(global, name = "timestamp_nanos", pure, return_raw)]
    pub fn timestamp_nanos(dt: &mut DateTimeFixed) -> Result<rhai::INT, Box<EvalAltResult>> {
        borrow_mut(dt).timestamp_nanos_opt().ok_or::<Box<EvalAltResult>>(
            Box::<EvalAltResult>::from("Timestamp out of range (range is ~584 years)".to_string())
        )
        .map(|nanos| nanos.into())
    }

    /// Returns the number of milliseconds since the last second boundary.
    #[rhai_fn(global, name = "timestamp_subsec_millis", pure)]
    pub fn timestamp_subsec_millis(dt: &mut DateTimeFixed) -> u32 {
        borrow_mut(dt).timestamp_subsec_millis()
    }

    /// Returns the number of microseconds since the last second boundary.
    #[rhai_fn(global, name = "timestamp_subsec_micros", pure)]
    pub fn timestamp_subsec_micros(dt: &mut DateTimeFixed) -> u32 {
        borrow_mut(dt).timestamp_subsec_micros()
    }

    /// Returns the number of nanoseconds since the last second boundary.
    #[rhai_fn(global, name = "timestamp_subsec_nanos", pure)]
    pub fn timestamp_subsec_nanos(dt: &mut DateTimeFixed) -> u32 {
        borrow_mut(dt).timestamp_subsec_nanos()
    }

    /// Retrieve the elapsed years from now to the given DateTime.
    #[rhai_fn(global, name = "years_since", pure)]
    pub fn years_since(dt: &mut DateTimeFixed, base: DateTimeFixed) -> i32 {
        if *base < **dt {
            (borrow_mut(&base).years_since(*borrow_mut(dt)).unwrap_or_default() as i32)*-1i32
        } else {
            borrow_mut(dt).years_since(*borrow_mut(&base)).unwrap_or_default() as i32
        }
    }

    /// Formats the combined date and time per the specified format string.
    ///
    /// See the [format::strftime](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) module for the supported escape sequences.
    #[rhai_fn(global, name = "format", pure)]
    pub fn format(dt: &mut DateTimeFixed, format: &str) -> String {
        format!("{}", borrow_mut(dt).format(format))
    }

    /// Formats the combined date and time per the specified format string and locale.
    ///
    /// See the [format::strftime](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) module on the supported escape sequences.
    /// See the [Locale](https://docs.rs/chrono/latest/chrono/enum.Locale.html) enum for list of valid locales
    #[rhai_fn(global, name = "format", pure, return_raw)]
    pub fn format_localized(dt: &mut DateTimeFixed, format: &str, locale: &str) -> Result<String, Box<EvalAltResult>> {
        let locale = Locale::from_str(locale).map_err(|_e| {
            Box::<EvalAltResult>::from(format!("Invalid locale provided: {}", locale))
        })?;

        Ok(format!("{}", borrow_mut(dt).format_localized(format, locale)))
    }

}
