// Generated macro for days_in_year_month (function)
macro_rules! Depcrate_utildays_in_year_month {
() => {
// Module: crate::util
// Provides: {"days_in_year_month"}
// Dependencies: {}
# [doc = " Get the number of days in the month of a given year."] # [doc = ""] # [doc = " ```rust"] # [doc = " # #![expect(deprecated)]"] # [doc = " # use time::{Month, util};"] # [doc = " assert_eq!(util::days_in_year_month(2020, Month::February), 29);"] # [doc = " ```"] # [deprecated (since = "0.3.37" , note = "use `days_in_month` or `Month::length` instead")] # [inline] pub const fn days_in_year_month (year : i32 , month : Month) -> u8 { days_in_month (month , year) }
};
}
