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

    /// Construct TimeDelta minimum
    #[rhai_fn(return_raw, name = "timedelta_min")]
    pub fn timedelta_min() -> Result<Timedelta, Box<EvalAltResult>> {
        Ok(Shared::new(Locked::new(TimeDelta::min_value())))
    }

    /// Construct TimeDelta maximum
    #[rhai_fn(return_raw, name = "timedelta_max")]
    pub fn timedelta_max() -> Result<Timedelta, Box<EvalAltResult>> {
        Ok(Shared::new(Locked::new(TimeDelta::max_value())))
    }

    /// Construct TimeDelta with number of seconds
    #[rhai_fn(return_raw, name = "timedelta_seconds")]
    pub fn timedelta_seconds(seconds: rhai::INT) -> Result<Timedelta, Box<EvalAltResult>> {
        Ok(Shared::new(Locked::new(TimeDelta::try_seconds(seconds).ok_or(Box::<EvalAltResult>::from("Delta out of range".to_string()))?)))
    }

    /// Construct TimeDelta with number of seconds
    #[rhai_fn(return_raw, name = "timedelta_seconds")]
    pub fn timedelta_seconds_and_nanos(seconds: rhai::INT, nanos: rhai::INT) -> Result<Timedelta, Box<EvalAltResult>> {
        Ok(Shared::new(Locked::new(TimeDelta::new(seconds, nanos as u32).ok_or(Box::<EvalAltResult>::from("Delta out of range".to_string()))?)))
    }

    /// Construct TimeDelta with number of days
    #[rhai_fn(return_raw, name = "timedelta_days")]
    pub fn timedelta_days(days: rhai::INT) -> Result<Timedelta, Box<EvalAltResult>> {
        Ok(Shared::new(Locked::new(TimeDelta::try_days(days).ok_or(Box::<EvalAltResult>::from("Delta out of range".to_string()))?)))
    }

    /// Construct TimeDelta with number of weeks
    #[rhai_fn(return_raw, name = "timedelta_weeks")]
    pub fn timedelta_weeks(weeks: rhai::INT) -> Result<Timedelta, Box<EvalAltResult>> {
        Ok(Shared::new(Locked::new(TimeDelta::try_weeks(weeks).ok_or(Box::<EvalAltResult>::from("Delta out of range".to_string()))?)))
    }

    /// Construct TimeDelta with number of hours
    #[rhai_fn(return_raw, name = "timedelta_hours")]
    pub fn timedelta_hours(hours: rhai::INT) -> Result<Timedelta, Box<EvalAltResult>> {
        Ok(Shared::new(Locked::new(TimeDelta::try_hours(hours).ok_or(Box::<EvalAltResult>::from("Delta out of range".to_string()))?)))
    }

    /// Construct TimeDelta with number of minutes
    #[rhai_fn(return_raw, name = "timedelta_minutes")]
    pub fn timedelta_minutes(minutes: rhai::INT) -> Result<Timedelta, Box<EvalAltResult>> {
        Ok(Shared::new(Locked::new(TimeDelta::try_minutes(minutes).ok_or(Box::<EvalAltResult>::from("Delta out of range".to_string()))?)))
    }

    /// Construct TimeDelta with number of milliseconds
    #[rhai_fn(return_raw, name = "timedelta_millis", name = "timedelta_milliseconds")]
    pub fn timedelta_millis(millis: rhai::INT) -> Result<Timedelta, Box<EvalAltResult>> {
        Ok(Shared::new(Locked::new(TimeDelta::try_milliseconds(millis).ok_or(Box::<EvalAltResult>::from("Delta out of range".to_string()))?)))
    }

    /// Construct TimeDelta with number of microseconds
    #[rhai_fn(name = "timedelta_micros", name = "timedelta_microseconds")]
    pub fn timedelta_micros(micros: rhai::INT) -> Timedelta {
        Shared::new(Locked::new(TimeDelta::microseconds(micros)))
    }

    /// Construct TimeDelta with number of microseconds
    #[rhai_fn(name = "timedelta_nanos", name = "timedelta_nanoseconds")]
    pub fn timedelta_nanos(micros: rhai::INT) -> Timedelta {
        Shared::new(Locked::new(TimeDelta::nanoseconds(micros)))
    }

    /// Check if is zero
    #[rhai_fn(global, get = "is_zero", name = "is_zero", pure)]
    pub fn is_zero(td: &mut Timedelta) -> bool {
        let this = borrow_mut(td);
        
        this.is_zero()
    }

    /// Convert to positive value
    #[rhai_fn(global, name = "abs", pure)]
    pub fn abs(td: &mut Timedelta) {
        let mut this = borrow_mut(td);
        
        *this = this.abs();
    }

    /// Add two deltas
    #[rhai_fn(global, name = "add", name = "plus", pure, return_raw)]
    pub fn add(td: &mut Timedelta, rhs: Timedelta) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(td);
        let rhs = *borrow_mut(&rhs);
        
        *this = this.checked_add(&rhs).ok_or(Box::<EvalAltResult>::from("Delta out of range".to_string()))?;

        Ok(())
    }

    /// Subtract two deltas
    #[rhai_fn(global, name = "sub", name = "minus", pure, return_raw)]
    pub fn sub(td: &mut Timedelta, rhs: Timedelta) -> Result<(), Box<EvalAltResult>> {
        let mut this = borrow_mut(td);
        let rhs = *borrow_mut(&rhs);
        
        *this = this.checked_sub(&rhs).ok_or(Box::<EvalAltResult>::from("Delta out of range".to_string()))?;

        Ok(())
    }

    /// Get the seconds delta 
    #[rhai_fn(global, get = "seconds", name = "seconds", name = "get_seconds", pure)]
    pub fn get_seconds(td: &mut Timedelta) -> rhai::INT {
        let this = borrow_mut(td);
        
        this.num_seconds() as rhai::INT
    }

    /// Get the minutes delta 
    #[rhai_fn(global, get = "minutes", name = "minutes", name = "get_minutes", pure)]
    pub fn get_minutes(td: &mut Timedelta) -> rhai::INT {
        let this = borrow_mut(td);
        
        this.num_minutes() as rhai::INT
    }

    /// Get the hours delta 
    #[rhai_fn(global, get = "hours", name = "hours", name = "get_hours", pure)]
    pub fn get_hours(td: &mut Timedelta) -> rhai::INT {
        let this = borrow_mut(td);
        
        this.num_hours() as rhai::INT
    }

    /// Get the days delta 
    #[rhai_fn(global, get = "days", name = "days", name = "get_days", pure)]
    pub fn get_days(td: &mut Timedelta) -> rhai::INT {
        let this = borrow_mut(td);
        
        this.num_days() as rhai::INT
    }

    /// Get the weeks delta 
    #[rhai_fn(global, get = "weeks", name = "weeks", name = "get_weeks", pure)]
    pub fn get_weeks(td: &mut Timedelta) -> rhai::INT {
        let this = borrow_mut(td);
        
        this.num_weeks() as rhai::INT
    }

    /// Get the subsec nanos delta 
    #[rhai_fn(global, get = "subsec_nanos", name = "subsec_nanos", name = "get_subsec_nanos", pure)]
    pub fn get_subsec_nanos(td: &mut Timedelta) -> rhai::INT {
        let this = borrow_mut(td);
        
        this.subsec_nanos() as rhai::INT
    }

    /// Get the milliseconds delta 
    #[rhai_fn(global, get = "milliseconds", name = "milliseconds", name = "get_milliseconds", pure)]
    pub fn get_milliseconds(td: &mut Timedelta) -> rhai::INT {
        let this = borrow_mut(td);
        
        this.num_milliseconds() as rhai::INT
    }

    /// Get the microseconds delta 
    #[rhai_fn(global, get = "microseconds", name = "microseconds", name = "get_microseconds", pure, return_raw)]
    pub fn get_microseconds(td: &mut Timedelta) -> Result<rhai::INT, Box<EvalAltResult>> {
        let this = borrow_mut(td);
        
        Ok(this.num_microseconds().ok_or(Box::<EvalAltResult>::from("Delta out of range".to_string()))? as rhai::INT)
    }

    /// Get the nanoseconds delta 
    #[rhai_fn(global, get = "nanoseconds", name = "nanoseconds", name = "get_nanoseconds", pure, return_raw)]
    pub fn get_nanoseconds(td: &mut Timedelta) -> Result<rhai::INT, Box<EvalAltResult>> {
        let this = borrow_mut(td);
        
        Ok(this.num_nanoseconds().ok_or(Box::<EvalAltResult>::from("Delta out of range".to_string()))? as rhai::INT)
    }

}