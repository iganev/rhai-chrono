#![allow(unused_imports)]
use std::ops::DerefMut;
use chrono::{DateTime, FixedOffset};
use rhai::plugin::*;
use rhai::{Locked, Shared};

#[inline(always)]
pub fn borrow_mut(datetime: &Shared<Locked<DateTime<FixedOffset>>>) -> impl DerefMut<Target = DateTime<FixedOffset>> + '_ {
    #[cfg(not(feature = "sync"))] return datetime.borrow_mut();

    #[cfg(feature = "sync")] return datetime.write().unwrap();
}

#[export_module]
pub mod datetime_module {

    use std::str::FromStr;

    use chrono::DateTime;
    use chrono::Days;
    use chrono::Months;
    use chrono::NaiveDateTime;
    use chrono::NaiveTime;
    use chrono::Datelike;
    use chrono::Timelike;
    use chrono::Local;
    use chrono::FixedOffset;
    use chrono::Utc;
    use chrono::Locale;


    use chrono_tz::Tz;
    use rhai::{EvalAltResult, Position, Shared, Locked};

    use crate::timedelta::timedelta_module::Timedelta;

    /// Alias type to bridge rhai and chrono DateTime
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

    /// Set timezone or offset
    #[rhai_fn(global, set = "timezone", name = "timezone", name = "set_timezone", name = "with_timezone", pure, return_raw)]
    pub fn set_timezone(dt: &mut DateTimeFixed, timezone: &str) -> Result<(), Box<EvalAltResult>> {
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

    /// Get timezone 
    #[rhai_fn(global, get = "timezone", name = "timezone", name = "get_timezone", pure)]
    pub fn get_timezone(dt: &mut DateTimeFixed) -> String {
        let this = borrow_mut(dt);
        
        this.timezone().to_string()
    }

    /// Get offset 
    #[rhai_fn(global, get = "offset", name = "offset", name = "get_offset", pure)]
    pub fn get_offset(dt: &mut DateTimeFixed) -> String {
        let this = borrow_mut(dt);
        
        this.offset().to_string()
    }

    /// Set the time segment with H:M:S formatted string; Defaults to midnight.
    #[rhai_fn(global, set = "time", name = "time", name = "set_time", name = "with_time", pure, return_raw)]
    pub fn set_time(dt: &mut DateTimeFixed, time: &str) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(dt);
        
        let time_segments: Vec<u32> = time.split(":").take(3).map(|v| v.parse().unwrap_or_default()).collect();

        let time = NaiveTime::from_hms_opt(time_segments.get(0).cloned().unwrap_or_default(), time_segments.get(1).cloned().unwrap_or_default(), time_segments.get(2).cloned().unwrap_or_default()).unwrap_or(NaiveTime::MIN);

        *this = this.with_time(time).unwrap();

        Ok(())
    }

    /// Get time 
    #[rhai_fn(global, get = "time", name = "time", name = "get_time", pure)]
    pub fn get_time(dt: &mut DateTimeFixed) -> String {
        let this = borrow_mut(dt);
        
        this.time().to_string()
    }

    /// Set the ordinal day
    #[rhai_fn(global, set = "ordinal", name = "ordinal", name = "set_ordinal", name = "with_ordinal", pure, return_raw)]
    pub fn set_ordinal(dt: &mut DateTimeFixed, day: rhai::INT) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(dt);
        
        *this = this.with_ordinal(day as u32).ok_or(Box::<EvalAltResult>::from("Day out of range or doesn't make any sense.".to_string()))?;

        Ok(())
    }

    /// Get ordinal 
    #[rhai_fn(global, get = "ordinal", name = "ordinal", name = "get_ordinal", pure)]
    pub fn get_ordinal(dt: &mut DateTimeFixed) -> rhai::INT {
        let this = borrow_mut(dt);
        
        this.ordinal() as rhai::INT
    }

    /// Set the ordinal0 day
    #[rhai_fn(global, set = "ordinal0", name = "ordinal0", name = "set_ordinal0", name = "with_ordinal0", pure, return_raw)]
    pub fn set_ordinal0(dt: &mut DateTimeFixed, day: rhai::INT) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(dt);
        
        *this = this.with_ordinal0(day as u32).ok_or(Box::<EvalAltResult>::from("Day out of range or doesn't make any sense.".to_string()))?;

        Ok(())
    }

    /// Get ordinal0 
    #[rhai_fn(global, get = "ordinal0", name = "ordinal0", name = "get_ordinal0", pure)]
    pub fn get_ordinal0(dt: &mut DateTimeFixed) -> rhai::INT {
        let this = borrow_mut(dt);
        
        this.ordinal0() as rhai::INT
    }

    /// Set the year
    #[rhai_fn(global, set = "year", name = "year", name = "set_year", name = "with_year", pure, return_raw)]
    pub fn set_year(dt: &mut DateTimeFixed, year: rhai::INT) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(dt);
        
        *this = this.with_year(year as i32).ok_or(Box::<EvalAltResult>::from("Year out of range or doesn't make any sense.".to_string()))?;

        Ok(())
    }

    /// Get the year 
    #[rhai_fn(global, get = "year", name = "year", name = "get_year", pure)]
    pub fn get_year(dt: &mut DateTimeFixed) -> rhai::INT {
        let this = borrow_mut(dt);
        
        this.year() as rhai::INT
    }

    /// Set the month
    #[rhai_fn(global, set = "month", name = "month", name = "set_month", name = "with_month", pure, return_raw)]
    pub fn set_month(dt: &mut DateTimeFixed, month: rhai::INT) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(dt);
        
        *this = this.with_month(month as u32).ok_or(Box::<EvalAltResult>::from("Month out of range or doesn't make any sense.".to_string()))?;

        Ok(())
    }

    /// Get the month 
    #[rhai_fn(global, get = "month", name = "month", name = "get_month", pure)]
    pub fn get_month(dt: &mut DateTimeFixed) -> rhai::INT {
        let this = borrow_mut(dt);
        
        this.month() as rhai::INT
    }

    /// Set the month0
    #[rhai_fn(global, set = "month0", name = "month0", name = "set_month0", name = "with_month0", pure, return_raw)]
    pub fn set_month0(dt: &mut DateTimeFixed, month0: rhai::INT) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(dt);
        
        *this = this.with_month0(month0 as u32).ok_or(Box::<EvalAltResult>::from("Month out of range or doesn't make any sense.".to_string()))?;

        Ok(())
    }

    /// Get the month0
    #[rhai_fn(global, get = "month0", name = "month0", name = "get_month0", pure)]
    pub fn get_month0(dt: &mut DateTimeFixed) -> rhai::INT {
        let this = borrow_mut(dt);
        
        this.month0() as rhai::INT
    }

    /// Set the day
    #[rhai_fn(global, set = "day", name = "day", name = "set_day", name = "with_day", pure, return_raw)]
    pub fn set_day(dt: &mut DateTimeFixed, day: rhai::INT) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(dt);
        
        *this = this.with_day(day as u32).ok_or(Box::<EvalAltResult>::from("Day out of range or doesn't make any sense.".to_string()))?;

        Ok(())
    }

    /// Get the day 
    #[rhai_fn(global, get = "day", name = "day", name = "get_day", pure)]
    pub fn get_day(dt: &mut DateTimeFixed) -> rhai::INT {
        let this = borrow_mut(dt);
        
        this.day() as rhai::INT
    }

    /// Set the day0
    #[rhai_fn(global, set = "day0", name = "day0", name = "set_day0", name = "with_day0", pure, return_raw)]
    pub fn set_day0(dt: &mut DateTimeFixed, day0: rhai::INT) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(dt);
        
        *this = this.with_day0(day0 as u32).ok_or(Box::<EvalAltResult>::from("Day out of range or doesn't make any sense.".to_string()))?;

        Ok(())
    }

    /// Get the day0
    #[rhai_fn(global, get = "day0", name = "day0", name = "get_day0", pure)]
    pub fn get_day0(dt: &mut DateTimeFixed) -> rhai::INT {
        let this = borrow_mut(dt);
        
        this.day0() as rhai::INT
    }

    /// Set the hour
    #[rhai_fn(global, set = "hour", name = "hour", name = "set_hour", name = "with_hour", pure, return_raw)]
    pub fn set_hour(dt: &mut DateTimeFixed, hour: rhai::INT) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(dt);
        
        *this = this.with_hour(hour as u32).ok_or(Box::<EvalAltResult>::from("Hour out of range or doesn't make any sense.".to_string()))?;

        Ok(())
    }

    /// Get the hour 
    #[rhai_fn(global, get = "hour", name = "hour", name = "get_hour", pure)]
    pub fn get_hour(dt: &mut DateTimeFixed) -> rhai::INT {
        let this = borrow_mut(dt);
        
        this.hour() as rhai::INT
    }

    /// Set the minute
    #[rhai_fn(global, set = "minute", name = "minute", name = "set_minute", name = "with_minute", pure, return_raw)]
    pub fn set_minute(dt: &mut DateTimeFixed, minute: rhai::INT) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(dt);
        
        *this = this.with_minute(minute as u32).ok_or(Box::<EvalAltResult>::from("Minute out of range or doesn't make any sense.".to_string()))?;

        Ok(())
    }

    /// Get the minute 
    #[rhai_fn(global, get = "minute", name = "minute", name = "get_minute", pure)]
    pub fn get_minute(dt: &mut DateTimeFixed) -> rhai::INT {
        let this = borrow_mut(dt);
        
        this.minute() as rhai::INT
    }

    /// Set the second
    #[rhai_fn(global, set = "second", name = "second", name = "set_second", name = "with_second", pure, return_raw)]
    pub fn set_second(dt: &mut DateTimeFixed, second: rhai::INT) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(dt);
        
        *this = this.with_second(second as u32).ok_or(Box::<EvalAltResult>::from("Seconds out of range or doesn't make any sense.".to_string()))?;

        Ok(())
    }

    /// Get the second 
    #[rhai_fn(global, get = "second", name = "second", name = "get_second", pure)]
    pub fn get_second(dt: &mut DateTimeFixed) -> rhai::INT {
        let this = borrow_mut(dt);
        
        this.second() as rhai::INT
    }

    /// Set the nanosecond
    #[rhai_fn(global, set = "nanosecond", name = "nanosecond", name = "set_nanosecond", name = "with_nanosecond", pure, return_raw)]
    pub fn set_nanosecond(dt: &mut DateTimeFixed, nanosecond: rhai::INT) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(dt);
        
        *this = this.with_nanosecond(nanosecond as u32).ok_or(Box::<EvalAltResult>::from("Nanoseconds out of range or doesn't make any sense.".to_string()))?;

        Ok(())
    }

    /// Get the nanosecond 
    #[rhai_fn(global, get = "nanosecond", name = "nanosecond", name = "get_nanosecond", pure)]
    pub fn get_nanosecond(dt: &mut DateTimeFixed) -> rhai::INT {
        let this = borrow_mut(dt);
        
        this.nanosecond() as rhai::INT
    }

    /// Add number of days
    #[rhai_fn(global, name = "add_days", name = "plus_days", pure, return_raw)]
    pub fn add_days(dt: &mut DateTimeFixed, days: rhai::INT) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(dt);
        
        *this = this.checked_add_days(Days::new(days as u64)).ok_or(Box::<EvalAltResult>::from("Days out of range or doesn't make any sense.".to_string()))?;

        Ok(())
    }

    /// Subtract number of days
    #[rhai_fn(global, name = "sub_days", name = "minus_days", pure, return_raw)]
    pub fn sub_days(dt: &mut DateTimeFixed, days: rhai::INT) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(dt);
        
        *this = this.checked_sub_days(Days::new(days as u64)).ok_or(Box::<EvalAltResult>::from("Days out of range or doesn't make any sense.".to_string()))?;

        Ok(())
    }

    /// Add number of months
    #[rhai_fn(global, name = "add_months", name = "plus_months", pure, return_raw)]
    pub fn add_months(dt: &mut DateTimeFixed, months: rhai::INT) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(dt);
        
        *this = this.checked_add_months(Months::new(months as u32)).ok_or(Box::<EvalAltResult>::from("Months out of range or doesn't make any sense.".to_string()))?;

        Ok(())
    }

    /// Subtract number of months
    #[rhai_fn(global, name = "sub_months", name = "minus_months", pure, return_raw)]
    pub fn sub_months(dt: &mut DateTimeFixed, months: rhai::INT) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(dt);
        
        *this = this.checked_sub_months(Months::new(months as u32)).ok_or(Box::<EvalAltResult>::from("Months out of range or doesn't make any sense.".to_string()))?;

        Ok(())
    }

    /// Add Timedelta
    #[rhai_fn(global, name = "add_timedelta", name = "plus_timedelta", pure, return_raw)]
    pub fn add_timedelta(dt: &mut DateTimeFixed, td: Timedelta) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(dt);
        let td = crate::timedelta::borrow_mut(&td);
        
        *this = this.checked_add_signed(*td).ok_or(Box::<EvalAltResult>::from("TimeDelta results in DateTime out of range or doesn't make any sense.".to_string()))?;

        Ok(())
    }

    /// Subtract Timedelta
    #[rhai_fn(global, name = "sub_timedelta", name = "minus_timedelta", pure, return_raw)]
    pub fn sub_timedelta(dt: &mut DateTimeFixed, td: Timedelta) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(dt);
        let td = crate::timedelta::borrow_mut(&td);
        
        *this = this.checked_sub_signed(*td).ok_or(Box::<EvalAltResult>::from("TimeDelta results in DateTime out of range or doesn't make any sense.".to_string()))?;

        Ok(())
    }

    /// Diff of two DateTime instances, producing TimeDelta (DateTime::signed_duration_since)
    #[rhai_fn(global, name = "diff", name = "cmp", name = "compare", name = "duration_since", name = "signed_duration_since", pure)]
    pub fn diff(dt: &mut DateTimeFixed, rhs: DateTimeFixed) -> Timedelta {
        let this = *borrow_mut(dt);
        let rhs = *borrow_mut(&rhs);

        Shared::new(Locked::new(this.signed_duration_since(&rhs)))
    }

}
