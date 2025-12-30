// Generated macro for use_587 (pub_use)
macro_rules! Depcrate_macrosuse_587 {
() => {
// Module: crate::macros
// Provides: {"use_587"}
// Dependencies: {}
# [doc = " Construct a [`PrimitiveDateTime`] or [`OffsetDateTime`] with a statically known value."] # [doc = ""] # [doc = " The resulting expression can be used in `const` or `static` declarations."] # [doc = ""] # [doc = " The syntax accepted by this macro is the same as [`date!`] and [`time!`], with an optional"] # [doc = " [`offset!`], all space-separated. If an [`offset!`] is provided, the resulting value will"] # [doc = " be an [`OffsetDateTime`]; otherwise it will be a [`PrimitiveDateTime`]."] # [doc = ""] # [doc = " [`OffsetDateTime`]: crate::OffsetDateTime"] # [doc = " [`PrimitiveDateTime`]: crate::PrimitiveDateTime"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time::{Date, Month, macros::datetime, UtcOffset};"] # [doc = " assert_eq!("] # [doc = "     datetime!(2020-01-01 0:00),"] # [doc = "     Date::from_calendar_date(2020, Month::January, 1)?.midnight()"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     datetime!(2020-01-01 0:00 UTC),"] # [doc = "     Date::from_calendar_date(2020, Month::January, 1)?.midnight().assume_utc()"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     datetime!(2020-01-01 0:00 -1),"] # [doc = "     Date::from_calendar_date(2020, Month::January, 1)?.midnight()"] # [doc = "         .assume_offset(UtcOffset::from_hms(-1, 0, 0)?)"] # [doc = " );"] # [doc = " # Ok::<_, time::Error>(())"] # [doc = " ```"] pub use time_macros :: datetime ;
};
}
