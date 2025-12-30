// Generated macro for is_leap_year (function)
macro_rules! Depcrate_utilis_leap_year {
() => {
// Module: crate::util
// Provides: {"is_leap_year"}
// Dependencies: {}
# [doc = " Returns if the provided year is a leap year in the proleptic Gregorian calendar. Uses"] # [doc = " [astronomical year numbering](https://en.wikipedia.org/wiki/Astronomical_year_numbering)."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time::util::is_leap_year;"] # [doc = " assert!(!is_leap_year(1900));"] # [doc = " assert!(is_leap_year(2000));"] # [doc = " assert!(is_leap_year(2004));"] # [doc = " assert!(!is_leap_year(2005));"] # [doc = " assert!(!is_leap_year(2100));"] # [doc = " ```"] # [inline] pub const fn is_leap_year (year : i32) -> bool { let d = if year % 100 == 0 { 15 } else { 3 } ; year & d == 0 }
};
}
