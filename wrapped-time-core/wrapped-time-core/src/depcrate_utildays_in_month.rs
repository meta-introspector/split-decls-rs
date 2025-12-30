// Generated macro for days_in_month (function)
macro_rules! Depcrate_utildays_in_month {
() => {
// Module: crate::util
// Provides: {"days_in_month"}
// Dependencies: {}
# [doc = " Get the number of days in the month of a given year."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time_core::util::days_in_month;"] # [doc = " assert_eq!(days_in_month(2, 2020), 29);"] # [doc = " ```"] # [doc = ""] # [doc = " Note: This function is not exposed by the `time` crate. It is an implementation detail."] # [inline] pub const fn days_in_month (month : u8 , year : i32) -> u8 { debug_assert ! (month >= 1) ; debug_assert ! (month <= 12) ; if hint :: unlikely (month == 2) { if is_leap_year (year) { 29 } else { 28 } } else { 30 | month ^ (month >> 3) } }
};
}
