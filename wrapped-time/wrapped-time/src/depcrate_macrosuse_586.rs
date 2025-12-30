// Generated macro for use_586 (pub_use)
macro_rules! Depcrate_macrosuse_586 {
() => {
// Module: crate::macros
// Provides: {"use_586"}
// Dependencies: {}
# [doc = " Construct a [`Date`](crate::Date) with a statically known value."] # [doc = ""] # [doc = " The resulting expression can be used in `const` or `static` declarations."] # [doc = ""] # [doc = " Three formats are supported: year-week-weekday, year-ordinal, and year-month-day."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time::{Date, Weekday::*, Month, macros::date};"] # [doc = " assert_eq!("] # [doc = "     date!(2020 - W 01 - 3),"] # [doc = "     Date::from_iso_week_date(2020, 1, Wednesday)?"] # [doc = " );"] # [doc = " assert_eq!(date!(2020-001), Date::from_ordinal_date(2020, 1)?);"] # [doc = " assert_eq!("] # [doc = "     date!(2020-01-01),"] # [doc = "     Date::from_calendar_date(2020, Month::January, 1)?"] # [doc = " );"] # [doc = " # Ok::<_, time::Error>(())"] # [doc = " ```"] pub use time_macros :: date ;
};
}
