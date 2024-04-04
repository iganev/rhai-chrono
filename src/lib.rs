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
    use rhai::Engine;
    use rhai::packages::Package;
    use crate::ChronoPackage;
    use crate::datetime::datetime_module::DateTimeFixed;

    #[test]
    fn it_works() {
        let mut engine = Engine::new();

        let package = ChronoPackage::new();
        package.register_into_engine(&mut engine);

        assert!(
            engine.eval::<DateTimeFixed>(r#"datetime_now()"#).is_ok(),
            "we should be getting Utc::now()"
        );

        assert!(
            engine.eval::<DateTimeFixed>(r#"datetime_utc()"#).is_ok(),
            "we should be getting Utc::now()"
        );

        assert!(
            engine.eval::<DateTimeFixed>(r#"datetime_local()"#).is_ok(),
            "we should be getting Local::now()"
        );

        assert!(
            engine.eval::<String>(r#"let dt = datetime_utc(); dt.to_string()"#).is_ok(),
            "we should be getting RFC3339 string"
        );

        assert!(
            engine.eval::<String>(r#"let dt = datetime_local(); dt.to_string()"#).is_ok(),
            "we should be getting RFC3339 string"
        );

        assert!(
            engine.eval::<String>(r#"let dt = datetime_utc(); dt.to_rfc3339()"#).is_ok(),
            "we should be getting RFC3339 string"
        );

        assert!(
            engine.eval::<String>(r#"let dt = datetime_local(); dt.to_rfc3339()"#).is_ok(),
            "we should be getting RFC3339 string"
        );


    }

    #[test]
    fn it_craps() {
        assert_ne!(0, 1);
    }
}
