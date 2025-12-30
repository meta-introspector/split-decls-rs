// Generated macro for days_in_month (function)
macro_rules! Depcrate_utildays_in_month {
() => {
// Module: crate::util
// Provides: {"days_in_month"}
// Dependencies: {}
# [doc = " Get the number of days in the month of a given year."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time::{Month, util};"] # [doc = " assert_eq!(util::days_in_month(Month::February, 2020), 29);"] # [doc = " ```"] # [inline] pub const fn days_in_month (month : Month , year : i32) -> u8 { time_core :: util :: days_in_month (month as u8 , year) }
};
}
