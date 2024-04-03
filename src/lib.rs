use rhai::def_package;
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
    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }

    #[test]
    fn it_craps() {
        assert_ne!(0, 1);
    }
}
