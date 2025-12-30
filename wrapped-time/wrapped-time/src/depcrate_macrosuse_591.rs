// Generated macro for use_591 (pub_use)
macro_rules! Depcrate_macrosuse_591 {
() => {
// Module: crate::macros
// Provides: {"use_591"}
// Dependencies: {}
# [doc = " Construct a [`UtcDateTime`] with a statically known value."] # [doc = ""] # [doc = " The resulting expression can be used in `const` or `static` declarations."] # [doc = ""] # [doc = " The syntax accepted by this macro is the same as a space-separated [`date!`] and [`time!`]."] # [doc = ""] # [doc = " [`UtcDateTime`]: crate::UtcDateTime"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time::{Date, Month, macros::utc_datetime};"] # [doc = " assert_eq!("] # [doc = "     utc_datetime!(2020-01-01 0:00),"] # [doc = "     Date::from_calendar_date(2020, Month::January, 1)?.midnight().as_utc()"] # [doc = " );"] # [doc = " # Ok::<_, time::Error>(())"] # [doc = " ```"] pub use time_macros :: utc_datetime ;
};
}
