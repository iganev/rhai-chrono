#![allow(unused_imports)]
use std::ops::DerefMut;
use chrono::TimeDelta;
use rhai::plugin::*;
use rhai::{Locked, Shared};

#[inline(always)]
pub fn borrow_mut(td: &Shared<Locked<TimeDelta>>) -> impl DerefMut<Target = TimeDelta> + '_ {
    #[cfg(not(feature = "sync"))] return td.borrow_mut();

    #[cfg(feature = "sync")] return td.write().unwrap();
}

#[export_module]
pub mod timedelta_module {
    use chrono::TimeDelta;

    use chrono_tz::Tz;
    use rhai::{EvalAltResult, Position, Shared, Locked};

    /// Alias type to bridge rhai and chrono TimeDelta
    pub type Timedelta = Shared<Locked<TimeDelta>>;

    /// Construct TimeDelta
    #[rhai_fn(return_raw, name = "timedelta", name = "timedelta_zero")]
    pub fn timedelta() -> Result<Timedelta, Box<EvalAltResult>> {
        Ok(Shared::new(Locked::new(TimeDelta::zero())))
    }

}