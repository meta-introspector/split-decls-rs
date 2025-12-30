// Generated macro for days_in_year (function)
macro_rules! Depcrate_utildays_in_year {
() => {
// Module: crate::util
// Provides: {"days_in_year"}
// Dependencies: {}
# [doc = " Get the number of calendar days in a given year."] # [doc = ""] # [doc = " The returned value will always be either 365 or 366."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time::util::days_in_year;"] # [doc = " assert_eq!(days_in_year(1900), 365);"] # [doc = " assert_eq!(days_in_year(2000), 366);"] # [doc = " assert_eq!(days_in_year(2004), 366);"] # [doc = " assert_eq!(days_in_year(2005), 365);"] # [doc = " assert_eq!(days_in_year(2100), 365);"] # [doc = " ```"] # [inline] pub const fn days_in_year (year : i32) -> u16 { if is_leap_year (year) { 366 } else { 365 } }
};
}
