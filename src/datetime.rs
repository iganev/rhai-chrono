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

    use chrono::{DateTime, Local, NaiveDateTime, NaiveTime};
    use chrono::FixedOffset;
    use chrono::Utc;
    use chrono::Locale;
    use chrono::Datelike;

    use chrono_tz::Tz;
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
    pub fn datetime_unix(secs: rhai::INT) -> Result<DateTimeFixed, Box<EvalAltResult>> {
        DateTime::from_timestamp(secs as i64, 0)
            .ok_or::<Box<EvalAltResult>>(
                Box::<EvalAltResult>::from("Timestamp out of range".to_string())
            )
            .map(|dt| Shared::new(Locked::new(dt.fixed_offset())))
    }

    /// Construct DateTime with UNIX timestamp in milliseconds
    #[rhai_fn(return_raw)]
    pub fn datetime_millis(millis: rhai::INT) -> Result<DateTimeFixed, Box<EvalAltResult>> {
        DateTime::from_timestamp_millis(millis as i64)
            .ok_or::<Box<EvalAltResult>>(
                Box::<EvalAltResult>::from("Timestamp out of range".to_string())
            )
            .map(|dt| Shared::new(Locked::new(dt.fixed_offset())))
    }

    /// Construct DateTime with UNIX timestamp in microseconds
    #[rhai_fn(return_raw)]
    pub fn datetime_micros(micros: rhai::INT) -> Result<DateTimeFixed, Box<EvalAltResult>> {
        DateTime::from_timestamp_micros(micros as i64)
            .ok_or::<Box<EvalAltResult>>(
                Box::<EvalAltResult>::from("Timestamp out of range".to_string())
            )
            .map(|dt| Shared::new(Locked::new(dt.fixed_offset())))
    }

    /// Construct DateTime with UNIX timestamp in nanoseconds
    #[rhai_fn(return_raw)]
    pub fn datetime_nanos(nanos: rhai::INT) -> Result<DateTimeFixed, Box<EvalAltResult>> {
        Ok(Shared::new(Locked::new(
            DateTime::from_timestamp_nanos(nanos as i64).fixed_offset(),
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
    pub fn timestamp(dt: &mut DateTimeFixed) -> rhai::INT {
        borrow_mut(dt).timestamp() as rhai::INT
    }

    /// Output UNIX timestamp in milliseconds
    #[rhai_fn(global, name = "timestamp_millis", pure)]
    pub fn timestamp_millis(dt: &mut DateTimeFixed) -> rhai::INT {
        borrow_mut(dt).timestamp_millis() as rhai::INT
    }

    /// Output UNIX timestamp in microseconds
    #[rhai_fn(global, name = "timestamp_micros", pure)]
    pub fn timestamp_micros(dt: &mut DateTimeFixed) -> rhai::INT {
        borrow_mut(dt).timestamp_micros() as rhai::INT
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
    pub fn timestamp_subsec_millis(dt: &mut DateTimeFixed) -> rhai::INT {
        borrow_mut(dt).timestamp_subsec_millis() as rhai::INT
    }

    /// Returns the number of microseconds since the last second boundary.
    #[rhai_fn(global, name = "timestamp_subsec_micros", pure)]
    pub fn timestamp_subsec_micros(dt: &mut DateTimeFixed) -> rhai::INT {
        borrow_mut(dt).timestamp_subsec_micros() as rhai::INT
    }

    /// Returns the number of nanoseconds since the last second boundary.
    #[rhai_fn(global, name = "timestamp_subsec_nanos", pure)]
    pub fn timestamp_subsec_nanos(dt: &mut DateTimeFixed) -> rhai::INT {
        borrow_mut(dt).timestamp_subsec_nanos() as rhai::INT
    }

    /// Retrieve the elapsed years from now to the given DateTime.
    #[rhai_fn(global, name = "years_since", pure)]
    pub fn years_since_now(dt: &mut DateTimeFixed) -> rhai::INT {
        let this = *borrow_mut(dt);
        let base = Local::now().fixed_offset();

        if base < this {
            this.years_since(base).map(|n| n as rhai::INT).unwrap_or_default()
        } else {
            base.years_since(this).map(|n| (n as rhai::INT)*-1).unwrap_or_default()
        }
    }

    /// Retrieve the elapsed years from given DateTime.
    #[rhai_fn(global, name = "years_since", pure)]
    pub fn years_since(dt: &mut DateTimeFixed, base: DateTimeFixed) -> rhai::INT {
        let this = *borrow_mut(dt);
        let base = *borrow_mut(&base);

        if base < this {
            this.years_since(base).map(|n| n as rhai::INT).unwrap_or_default()
        } else {
            base.years_since(this).map(|n| (n as rhai::INT)*-1).unwrap_or_default()
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
    /// 
    /// See the [Locale](https://docs.rs/chrono/latest/chrono/enum.Locale.html) enum for list of valid locales
    #[rhai_fn(global, name = "format", pure, return_raw)]
    pub fn format_localized(dt: &mut DateTimeFixed, format: &str, locale: &str) -> Result<String, Box<EvalAltResult>> {
        let locale = Locale::from_str(locale).map_err(|_e| {
            Box::<EvalAltResult>::from(format!("Invalid locale provided: {}", locale))
        })?;

        Ok(format!("{}", borrow_mut(dt).format_localized(format, locale)))
    }

    #[rhai_fn(global, name = "timezone", name = "set_timezone", name = "with_timezone", pure, return_raw)]
    pub fn timezone(dt: &mut DateTimeFixed, timezone: &str) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(dt);
        
        let tz: FixedOffset = if timezone.to_lowercase() == "local" {
            Local::now().fixed_offset().timezone()
        } else if timezone.contains('0') {
            if let Ok(tz) = FixedOffset::from_str(&timezone) {
                tz
            } else {
                return Err(Box::<EvalAltResult>::from("Failed to parse timezone offset. Supported values are IANA timezones, local or valid fixed offset".to_string()));
            }
        } else {
            if let Ok(tz) = timezone.parse::<Tz>() {
                this.with_timezone(&tz).fixed_offset().timezone()
            } else {
                return Err(Box::<EvalAltResult>::from("Failed to parse IANA timezone. Supported values are IANA timezones, local or valid fixed offset".to_string()));
            }
        };

        *this = this.with_timezone(&tz);

        Ok(())
    }

    /// Set the time segment with H:M:S formatted string; Defaults to midnight.
    #[rhai_fn(global, name = "time", name = "set_time", name = "with_time", pure, return_raw)]
    pub fn time(dt: &mut DateTimeFixed, time: &str) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(dt);
        
        let time_segments: Vec<u32> = time.split(":").take(3).map(|v| v.parse().unwrap_or_default()).collect();

        let time = NaiveTime::from_hms_opt(time_segments.get(0).cloned().unwrap_or_default(), time_segments.get(1).cloned().unwrap_or_default(), time_segments.get(2).cloned().unwrap_or_default()).unwrap_or(NaiveTime::MIN);

        *this = this.with_time(time).unwrap();

        Ok(())

    }

    /// Set the time segment with H:M:S formatted string; Defaults to midnight.
    #[rhai_fn(global, name = "ordinal", name = "set_ordinal", name = "with_ordinal", pure, return_raw)]
    pub fn ordinal(dt: &mut DateTimeFixed, day: rhai::INT) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(dt);
        
        *this = this.with_ordinal(day as u32).ok_or(Box::<EvalAltResult>::from("Day out of range or doesn't make any sense.".to_string()))?;

        Ok(())
    }

    /// Set the time segment with H:M:S formatted string; Defaults to midnight.
    #[rhai_fn(global, name = "ordinal0", name = "set_ordinal0", name = "with_ordinal0", pure, return_raw)]
    pub fn ordinal0(dt: &mut DateTimeFixed, day: rhai::INT) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(dt);
        
        *this = this.with_ordinal0(day as u32).ok_or(Box::<EvalAltResult>::from("Day out of range or doesn't make any sense.".to_string()))?;

        Ok(())
    }

}
