[![Crates.io](https://img.shields.io/crates/v/rhai-chrono?color=4d76ae)](https://crates.io/crates/rhai-chrono)
[![API](https://docs.rs/rhai-chrono/badge.svg)](https://docs.rs/rhai-chrono)
[![dependency status](https://deps.rs/repo/github/iganev/rhai-chrono/status.svg)](https://deps.rs/repo/github/iganev/rhai-chrono)
[![build and test](https://github.com/iganev/rhai-chrono/actions/workflows/rust.yml/badge.svg)](https://github.com/iganev/rhai-chrono/actions/workflows/rust.yml)
[![codecov](https://codecov.io/github/iganev/rhai-chrono/graph/badge.svg?token=B5P2TAV5BB)](https://codecov.io/github/iganev/rhai-chrono)


# rhai-chrono
[chrono](https://github.com/chronotope/chrono) [DateTime](https://docs.rs/chrono/latest/chrono/struct.DateTime.html) and [TimeDelta](https://docs.rs/chrono/latest/chrono/struct.TimeDelta.html) package for [rhai](https://github.com/rhaiscript/rhai/tree/main)

## Quick Start

Developed and tested with rhai v1.17, chrono v0.4.37 and chrono-tz v0.9.

### Include

Add to `Cargo.toml`:
```toml
rhai = { version = "^1.15" } # in case you've missed it
rhai-chrono = { version = "^0" }
```

### Registration

Include package:
```rust
use rhai::Engine;
use rhai_chrono::ChronoPackage;
```

Register package:
```rust
let mut engine = Engine::new();

let package = ChronoPackage::new();
package.register_into_engine(&mut engine);
```

### Behavior

The package exposes two wrapper types `DateTimeFixed` (wrapping `chrono::DateTime<FixedOffset>`) and `Timedelta` (wrapping `chrono::TimeDelta`).

Each of the two wrapper types can be initialized in a variety of ways using disctinct contructor functions.

Once initialized, the user can call methods and get / set properties on the wrapper.

Certain methods yield output values, either a number, string or boolean.

Many of the methods and properties have aliases or variants for convenience.

All methods and properties follow (but with slight nuance) the way you'd normally use `chrono::DateTime` and `chrono::TimeDelta`.

## API

### DateTime

All `DateTime` constructors create a `DateTime<FixedOffset>` internally. This current limitation is worth noting, when working with timezones.

#### Constructors

`datetime_utc()` | `datetime_now()`: creates a new DateTime set to current UTC time.

`datetime_local()`: creates a new DateTime set to current local time.

`datetime_unix(i64)`: creates a new DateTime set to the given UNIX timestamp in seconds.

`datetime_millis(i64)`: creates a new DateTime set to the given UNIX timestamp in milliseconds.

`datetime_micros(i64)`: creates a new DateTime set to the given UNIX timestamp in microseconds.

`datetime_nanos(i64)`: creates a new DateTime set to the given UNIX timestamp in nanoseconds.

`datetime_rfc2822(String)`: creates a new DateTime using a valid RFC2822 string.

`datetime_rfc3339(String)`: creates a new DateTime using a valid RFC3339 string.

`datetime_parse(timestamp: String, format: String)`: creates a new DateTime using a custom timestamp and [strftime](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) syntax format.



#### Setters

`timezone = String`: accepts valid [IANA timezone name](https://docs.rs/chrono-tz/latest/chrono_tz/), "local" or valid offset (i.e. "-06:00").

`time = String`: accepts a `%H:%M:%S` formatted `string`.

`ordinal = i64`:  sets the day of the year starting from 1 being Jan 1st.

`ordinal0 = i64`:  sets the day of the year starting from 0 being Jan 1st.

`year = i64`: sets the year

`month = i64`: sets the month starting from 1 being Jan.

`month0 = i64`: sets the month starting from 0 being Jan.

`day = i64`: sets the day of the month starting from 1 being the 1st.

`day0 = i64`: sets the day of the month starting from 0 being the 1st.

`hour = i64`: sets the hour of the day.

`minute = i64`: sets the minute of the hour.

`second = i64`: sets the second of the minute.

`nanosecond = i64`: sets the nanosecond beyond the last complete second.

#### Getters

`timezone` | `offset`: returns a `string` representation of the currently set **offset**.

`time`: returns a `string` representation of the current time segment in `%H:%M:%S` format.

`ordinal`: returns an `i64` ordinal day of the year starting from 1 being Jan 1st.

`ordinal0`: returns an `i64` ordinal day of the year starting from 0 being Jan 1st.

`year`: gets the year.

`month`: gets the month starting from 1 being Jan.

`month0`: gets the month starting from 0 being Jan.

`day`: gets the day of the month starting from 1 being the 1st.

`day0`: gets the day of the month starting from 0 being the 1st.

`hour`: gets the hour of the day.

`minute`: gets the minute of the hour.

`second`: gets the second of the minute.

`nanosecond`: gets the nanosecond beyond the last complete second.

#### Methods

`to_string()` | `to_rfc3339()`: returns a `string`` in RFC3339 format.

`to_rfc2822()`: returns a `string` in RFC2822 format.

`timestamp()`: returns an `i64` UNIX timestamp.

`timestamp_millis()`: returns an `i64` UNIX timestamp in milliseconds.

`timestamp_micros()`: returns an `i64` UNIX timestamp in microseconds.

`timestamp_nanos()`: returns an `i64` UNIX timestamp in nanoseconds or an out-of-range error.

`timestamp_subsec_millis()`: returns an `i64` amount of milliseconds beyond the last complete second.

`timestamp_subsec_micros()`: returns an `i64` amount of microseconds beyond the last complete second.

`timestamp_subsec_nanos()`: returns an `i64` amount of nanoseconds beyond the last complete second.

`years_since(Optional DateTimeFixed)`: returns a **SIGNED** `i64` number of years difference. If first parameter is not supplied, current local time is used for comparison.

`format(format: String, Optional locale: String)`: returns a custom formatted timestamp. Format parameter must be in [strftime](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) syntax format. Optionally, the user can supply a valid [locale](https://docs.rs/chrono/latest/chrono/enum.Locale.html) name as string.

`timezone(String) | set_timezone(String) | with_timezone(String)`: sets the **offset** based on a valid [IANA timezone name](https://docs.rs/chrono-tz/latest/chrono_tz/), "local" or valid offset (i.e. "-06:00").

`timezone() | offset() | get_timezone() | get_offset()`: returns a `string` representation of the currently set **offset**.

`time(String)` | `set_time(String)` | `with_time(String)`: sets the time segment using a `%H:%M:%S` formatted `string`.

`time() | get_time()`: returns a `string` representation of the current time segment in `%H:%M:%S` format.

`ordinal(i64) | set_ordinal(i64) | with_ordinal(i64)`: sets the day of the year starting from 1 being Jan 1st.

`ordinal() | get_ordinal()`: returns an `i64` ordinal day of the year starting from 1 being Jan 1st.

`ordinal0(i64) | set_ordinal0(i64) | with_ordinal0(i64)`: sets the day of the year starting from 0 being Jan 1st.

`ordinal0() | get_ordinal0()`: returns an `i64` ordinal day of the year starting from 0 being Jan 1st.

`year(i64) | set_year(i64) | with_year(i64)`: sets the year.

`year() | get_year()`: gets the year.

`month(i64) | set_month(i64) | with_month(i64)`: sets the month starting from 1 being Jan.

`month() | get_month()`: gets the month starting from 1 being Jan.

`month0(i64) | set_month0(i64) | with_month0(i64)`: sets the month starting from 1 being Jan.

`month0() | get_month0()`: gets the month starting from 1 being Jan.

`day(i64) | set_day(i64) | with_day(i64)`: sets the day of the month starting from 1 being the 1st.

`day() | get_day()`: gets the day of the month starting from 1 being the 1st.

`day0(i64) | set_day0(i64) | with_day0(i64)`: sets the day of the month starting from 0 being 1st.

`day0() | get_day0()`: gets the day of the month starting from 0 being 1st.

`hour(i64) | set_hour(i64) | with_hour(i64)`: sets the hour of the day.

`hour() | get_hour()`: gets the hour of the day.

`minute(i64) | set_minute(i64) | with_minute(i64)`: sets the minute of the hour.

`minute() | get_minute()`: gets the minute of the hour.

`second(i64) | set_second(i64) | with_second(i64)`: sets the second of the minute.

`second() | get_second()`: gets the second of the minute.

`nanosecond(i64) | set_nanosecond(i64) | with_nanosecond(i64)`: sets the nanosecond beyond the last complete second.

`nanosecond() | get_nanosecond()`: gets the nanosecond beyond the last complete second.

`add_days(i64) | plus_days(i64)`: adds a given number of days to the DateTime.

`sub_days(i64) | minus_days(i64)`: subtracts a given number of days to the DateTime.

`add_months(i64) | plus_months(i64)`: adds a given number of months to the DateTime.

`sub_months(i64) | minus_months(i64)`: subtracts a given number of months to the DateTime.

`add_timedelta(Timedelta) | plus_timedelta(Timedelta)`: adds a `Timedelta` to the DateTime.

`sub_timedelta(Timedelta) | minus_timedelta(Timedelta)`: subtracts a `Timedelta` from the DateTime.

`diff(DateTimeFixed) | cmp(DateTimeFixed) | compare(DateTimeFixed) | duration_since(DateTimeFixed) | signed_duration_since(DateTimeFixed)`: calculates the difference between two `DateTimeFixed` instances and returns a `Timedelta`.

### TimeDelta

#### Constructors

#### Setters

#### Getters

#### Methods

## Examples

TODO

## License

This library (rhai-chrono) is open sourced under the BSD 2 License.