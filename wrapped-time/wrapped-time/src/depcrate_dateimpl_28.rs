// Generated macro for impl_28 (impl)
macro_rules! Depcrate_dateimpl_28 {
() => {
// Module: crate::date
// Provides: {"impl_28"}
// Dependencies: {}
# [cfg (feature = "parsing")] impl Date { # [doc = " Parse a `Date` from the input using the provided [format"] # [doc = " description](crate::format_description)."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time::Date;"] # [doc = " # use time_macros::{date, format_description};"] # [doc = " let format = format_description!(\"[year]-[month]-[day]\");"] # [doc = " assert_eq!(Date::parse(\"2020-01-02\", &format)?, date!(2020-01-02));"] # [doc = " # Ok::<_, time::Error>(())"] # [doc = " ```"] # [inline] pub fn parse (input : & str , description : & (impl Parsable + ? Sized) ,) -> Result < Self , error :: Parse > { description . parse_date (input . as_bytes ()) } }
};
}
