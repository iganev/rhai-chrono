use rhai::{def_package};
use rhai::plugin::*;

pub(crate) mod datetime;

def_package! {
    /// Package for chrono datetime usage.
    pub ChronoPackage(lib) {
       combine_with_exported_module!(lib, "rhai_chrono_datetime", datetime::datetime_module);
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Local, Utc};
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
            engine.eval::<u32>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.timestamp_subsec_millis()"#, timestamp_rfc3339_nanos)).unwrap_or_default(),
            123u32,
            "we should be getting UNIX milliseconds remainder"
        );

        // test timestamp_subsec_micros
        assert_eq!(
            engine.eval::<u32>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.timestamp_subsec_micros()"#, timestamp_rfc3339_nanos)).unwrap_or_default(),
            123456u32,
            "we should be getting UNIX microseconds remainder"
        );

        // test timestamp_subsec_nanos
        assert_eq!(
            engine.eval::<u32>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.timestamp_subsec_nanos()"#, timestamp_rfc3339_nanos)).unwrap_or_default(),
            123456789u32,
            "we should be getting UNIX nanoseconds remainder"
        );

        // test years_since
        let years_since = Local::now().fixed_offset().years_since(DateTime::from_timestamp_nanos(timestamp_unix_nanos as i64).fixed_offset()).unwrap_or_default() as i32;

        assert_eq!(
            engine.eval::<i32>(&format!(r#"let dt = datetime_now(); dt.years_since(datetime_rfc3339("{}"))"#, timestamp_rfc3339_nanos)).unwrap_or_default(),
            years_since,
            "we should be getting number of years"
        );

        assert_eq!(
            engine.eval::<i32>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.years_since()"#, timestamp_rfc3339_nanos)).unwrap_or_default(),
            years_since*-1i32,
            "we should be getting number of years"
        );

        assert_eq!(
            engine.eval::<i32>(&format!(r#"let dt = datetime_rfc3339("{}"); dt.years_since(datetime_now())"#, timestamp_rfc3339_nanos)).unwrap_or_default(),
            years_since*-1i32,
            "we should be getting number of years"
        );

        // test format

        // test format + locale

    }

    #[test]
    fn it_craps() {
        assert_ne!(0, 1);
    }
}
