use rhai::def_package;
use rhai::plugin::*;

pub(crate) mod datetime;
pub(crate) mod timedelta;

def_package! {
    /// Package for chrono datetime usage.
    pub ChronoPackage(lib) {
       combine_with_exported_module!(lib, "rhai_chrono_datetime", datetime::datetime_module);
       combine_with_exported_module!(lib, "rhai_chrono_timedelta", timedelta::timedelta_module);
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Datelike, Timelike, Local, NaiveTime};

    use rhai::Engine;
    use rhai::packages::Package;
    
    use crate::ChronoPackage;
    use crate::datetime::datetime_module::DateTimeFixed;

    #[test]
    fn it_works() {
        let mut engine = Engine::new();

        let package = ChronoPackage::new();
        package.register_into_engine(&mut engine);

        let timestamp_unix: u64 = 618658211;
        let timestamp_unix_millis: u64 = 618658211123;
        let timestamp_unix_micros: u64 = 618658211123456;
        let timestamp_unix_nanos: u64 = 618658211123456789;
        let timestamp_mysql = "1989-08-09 09:30:11";
        let timestamp_mysql_format = "%Y-%m-%d %H:%M:%S";
        let timestamp_rfc2822 = "Wed, 9 Aug 1989 09:30:11 +0000";
        let timestamp_rfc3339 = "1989-08-09T09:30:11+00:00";
        let timestamp_rfc3339_millis = "1989-08-09T09:30:11.123+00:00";
        let timestamp_rfc3339_micros = "1989-08-09T09:30:11.123456+00:00";
        let timestamp_rfc3339_nanos = "1989-08-09T09:30:11.123456789+00:00";
        let timestamp_localized = "mercredi, août  9";
        let timestamp_localized_format = "%A, %B %e";
        let timestamp_localized_locale = "fr_FR";

        // test init now
        assert!(
            engine.eval::<DateTimeFixed>(r#"datetime_now()"#).is_ok(),
            "we should be getting Utc::now()"
        );

        // test init utc
        assert!(
            engine.eval::<DateTimeFixed>(r#"datetime_utc()"#).is_ok(),
            "we should be getting Utc::now()"
        );

        // test init local
        assert!(
            engine.eval::<DateTimeFixed>(r#"datetime_local()"#).is_ok(),
            "we should be getting Local::now()"
        );

        // test to_string from utc
        assert!(
            engine.eval::<String>(r#"let dt = datetime_utc(); dt.to_string()"#).is_ok(),
            "we should be getting RFC3339 string"
        );

        // test to string from local
        assert!(
            engine.eval::<String>(r#"let dt = datetime_local(); dt.to_string()"#).is_ok(),
            "we should be getting RFC3339 string"
        );

        // test init from unix timestamp
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_unix({}); dt.to_string()"#, timestamp_unix)).unwrap_or_default(),
            timestamp_rfc3339,
            "we should be getting RFC3339 string"
        );

        // test init from unix millis
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_millis({}); dt.to_string()"#, timestamp_unix_millis)).unwrap_or_default(),
            timestamp_rfc3339_millis,
            "we should be getting RFC3339 string"
        );

        // test init from unix micros
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_micros({}); dt.to_string()"#, timestamp_unix_micros)).unwrap_or_default(),
            timestamp_rfc3339_micros,
            "we should be getting RFC3339 string"
        );

        // test init from unix nanos
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_nanos({}); dt.to_string()"#, timestamp_unix_nanos)).unwrap_or_default(),
            timestamp_rfc3339_nanos,
            "we should be getting RFC3339 string"
        );

        // test init from rfc2822
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_rfc2822("{}"); dt.to_string()"#, timestamp_rfc2822)).unwrap_or_default(),
            timestamp_rfc3339,
            "we should be getting RFC3339 string"
        );

        // test init from rfc3339
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.to_string()"#, timestamp_rfc3339)).unwrap_or_default(),
            timestamp_rfc3339,
            "we should be getting RFC3339 string"
        );

        // test init from string and format
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_parse("{}", "{}"); dt.to_string()"#, timestamp_mysql, timestamp_mysql_format)).unwrap_or_default(),
            timestamp_rfc3339,
            "we should be getting RFC3339 string"
        );

        // test to_rfc3339 from utc
        assert!(
            engine.eval::<String>(r#"let dt = datetime_utc(); dt.to_rfc3339()"#).is_ok(),
            "we should be getting RFC3339 string"
        );

        // test to_rfc3339
        assert!(
            engine.eval::<String>(r#"let dt = datetime_local(); dt.to_rfc3339()"#).is_ok(),
            "we should be getting RFC3339 string"
        );

        // test to_rfc2822
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.to_rfc2822()"#, timestamp_rfc3339)).unwrap_or_default(),
            timestamp_rfc2822,
            "we should be getting RFC2822 string"
        );

        // test timestamp
        assert_eq!(
            engine.eval::<i64>(&format!(r#"let dt = datetime_rfc2822("{}"); dt.timestamp()"#, timestamp_rfc2822)).unwrap_or_default(),
            timestamp_unix as i64,
            "we should be getting UNIX timestamp i64"
        );

        // test timestamp_millis
        assert_eq!(
            engine.eval::<i64>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.timestamp_millis()"#, timestamp_rfc3339_millis)).unwrap_or_default(),
            timestamp_unix_millis as i64,
            "we should be getting UNIX milliseconds timestamp i64"
        );

        // test timestamp_micros
        assert_eq!(
            engine.eval::<i64>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.timestamp_micros()"#, timestamp_rfc3339_micros)).unwrap_or_default(),
            timestamp_unix_micros as i64,
            "we should be getting UNIX microseconds timestamp i64"
        );

        // test timestamp_nanos
        assert_eq!(
            engine.eval::<i64>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.timestamp_nanos()"#, timestamp_rfc3339_nanos)).unwrap_or_default(),
            timestamp_unix_nanos as i64,
            "we should be getting UNIX nanoseconds timestamp i64"
        );

        // test timestamp_subsec_millis
        assert_eq!(
            engine.eval::<rhai::INT>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.timestamp_subsec_millis()"#, timestamp_rfc3339_nanos)).unwrap_or_default(),
            123u32 as rhai::INT,
            "we should be getting UNIX milliseconds remainder"
        );

        // test timestamp_subsec_micros
        assert_eq!(
            engine.eval::<rhai::INT>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.timestamp_subsec_micros()"#, timestamp_rfc3339_nanos)).unwrap_or_default(),
            123456u32 as rhai::INT,
            "we should be getting UNIX microseconds remainder"
        );

        // test timestamp_subsec_nanos
        assert_eq!(
            engine.eval::<rhai::INT>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.timestamp_subsec_nanos()"#, timestamp_rfc3339_nanos)).unwrap_or_default(),
            123456789u32 as rhai::INT,
            "we should be getting UNIX nanoseconds remainder"
        );

        // test years_since
        let years_since = Local::now().fixed_offset().years_since(DateTime::from_timestamp_nanos(timestamp_unix_nanos as i64).fixed_offset()).unwrap_or_default() as i32;

        assert_eq!(
            engine.eval::<rhai::INT>(&format!(r#"let dt = datetime_now(); dt.years_since(datetime_rfc3339("{}"))"#, timestamp_rfc3339_nanos)).unwrap_or_default(),
            years_since as rhai::INT,
            "we should be getting number of years"
        );

        assert_eq!(
            engine.eval::<rhai::INT>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.years_since()"#, timestamp_rfc3339_nanos)).unwrap_or_default(),
            (years_since*-1i32) as rhai::INT,
            "we should be getting number of years"
        );

        assert_eq!(
            engine.eval::<rhai::INT>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.years_since(datetime_now())"#, timestamp_rfc3339_nanos)).unwrap_or_default(),
            (years_since*-1i32) as rhai::INT,
            "we should be getting number of years"
        );

        // test format
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_rfc2822("{}"); dt.format("{}")"#, timestamp_rfc2822, timestamp_mysql_format)).unwrap_or_default(),
            timestamp_mysql,
            "we should be getting MySQL datetime string"
        );

        // test format + locale
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_rfc2822("{}"); dt.format("{}", "{}")"#, timestamp_rfc2822, timestamp_localized_format, timestamp_localized_locale)).unwrap_or_default(),
            timestamp_localized,
            "we should be getting pretty french words"
        );

        // test with timezone local
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.timezone("local"); dt.to_rfc2822()"#, timestamp_rfc3339)).unwrap_or_default(),
            DateTime::parse_from_rfc2822(timestamp_rfc2822).unwrap().with_timezone(&Local::now().fixed_offset().timezone()).to_rfc2822(),
            "we should be getting RFC2822 string"
        );

        // test with IANA timezone
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.timezone("America/Edmonton"); dt.to_rfc2822()"#, timestamp_rfc3339)).unwrap_or_default(),
            DateTime::parse_from_rfc2822(timestamp_rfc2822).unwrap().with_timezone(&chrono_tz::America::Edmonton).to_rfc2822(),
            "we should be getting RFC2822 string"
        );

        // test with offset for timezone
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.timezone("-06:00"); dt.to_rfc2822()"#, timestamp_rfc3339)).unwrap_or_default(),
            DateTime::parse_from_rfc2822(timestamp_rfc2822).unwrap().with_timezone(&chrono_tz::America::Edmonton).to_rfc2822(),
            "we should be getting RFC2822 string"
        );

        // test with time
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.time("12:15"); dt.to_rfc2822()"#, timestamp_rfc3339)).unwrap_or_default(),
            DateTime::parse_from_rfc2822(timestamp_rfc2822).unwrap().with_time(NaiveTime::from_hms_opt(12, 15, 0).unwrap()).unwrap().to_rfc2822(),
            "we should be getting RFC2822 string"
        );

        // test ordinal
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.ordinal(5); dt.to_rfc2822()"#, timestamp_rfc3339)).unwrap_or_default(),
            DateTime::parse_from_rfc2822(timestamp_rfc2822).unwrap().with_ordinal(5).unwrap().to_rfc2822(),
            "we should be getting RFC2822 string"
        );

        // test ordinal0
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.ordinal0(5); dt.to_rfc2822()"#, timestamp_rfc3339)).unwrap_or_default(),
            DateTime::parse_from_rfc2822(timestamp_rfc2822).unwrap().with_ordinal0(5).unwrap().to_rfc2822(),
            "we should be getting RFC2822 string"
        );

        // test year
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.year(1990); dt.to_rfc2822()"#, timestamp_rfc3339)).unwrap_or_default(),
            DateTime::parse_from_rfc2822(timestamp_rfc2822).unwrap().with_year(1990).unwrap().to_rfc2822(),
            "we should be getting RFC2822 string"
        );

        // test month
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.month(11); dt.to_rfc2822()"#, timestamp_rfc3339)).unwrap_or_default(),
            DateTime::parse_from_rfc2822(timestamp_rfc2822).unwrap().with_month(11).unwrap().to_rfc2822(),
            "we should be getting RFC2822 string"
        );

        // test month0
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.month0(10); dt.to_rfc2822()"#, timestamp_rfc3339)).unwrap_or_default(),
            DateTime::parse_from_rfc2822(timestamp_rfc2822).unwrap().with_month0(10).unwrap().to_rfc2822(),
            "we should be getting RFC2822 string"
        );

        // test day
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.day(11); dt.to_rfc2822()"#, timestamp_rfc3339)).unwrap_or_default(),
            DateTime::parse_from_rfc2822(timestamp_rfc2822).unwrap().with_day(11).unwrap().to_rfc2822(),
            "we should be getting RFC2822 string"
        );

        // test day0
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.day0(10); dt.to_rfc2822()"#, timestamp_rfc3339)).unwrap_or_default(),
            DateTime::parse_from_rfc2822(timestamp_rfc2822).unwrap().with_day0(10).unwrap().to_rfc2822(),
            "we should be getting RFC2822 string"
        );

        // test hour
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.hour(23); dt.to_rfc2822()"#, timestamp_rfc3339)).unwrap_or_default(),
            DateTime::parse_from_rfc2822(timestamp_rfc2822).unwrap().with_hour(23).unwrap().to_rfc2822(),
            "we should be getting RFC2822 string"
        );

        // test minute
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.minute(33); dt.to_rfc2822()"#, timestamp_rfc3339)).unwrap_or_default(),
            DateTime::parse_from_rfc2822(timestamp_rfc2822).unwrap().with_minute(33).unwrap().to_rfc2822(),
            "we should be getting RFC2822 string"
        );

        // test second
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.second(7); dt.to_rfc2822()"#, timestamp_rfc3339)).unwrap_or_default(),
            DateTime::parse_from_rfc2822(timestamp_rfc2822).unwrap().with_second(7).unwrap().to_rfc2822(),
            "we should be getting RFC2822 string"
        );

        // test nanosecond
        assert_eq!(
            engine.eval::<String>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.nanosecond(123456789); dt.to_rfc2822()"#, timestamp_rfc3339)).unwrap_or_default(),
            DateTime::parse_from_rfc2822(timestamp_rfc2822).unwrap().with_nanosecond(123456789).unwrap().to_rfc2822(),
            "we should be getting RFC2822 string"
        );


    }

    #[test]
    fn it_craps() {
        assert_ne!(0, 1);
    }
}
