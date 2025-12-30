// Generated macro for impl_1074 (impl)
macro_rules! Depcrate_utc_date_timeimpl_1074 {
() => {
// Module: crate::utc_date_time
// Provides: {"impl_1074"}
// Dependencies: {}
# [cfg (feature = "parsing")] impl UtcDateTime { # [doc = " Parse an `UtcDateTime` from the input using the provided [format"] # [doc = " description](crate::format_description). A [`UtcOffset`] is permitted, but not required to"] # [doc = " be present. If present, the value will be converted to UTC."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time::UtcDateTime;"] # [doc = " # use time_macros::{utc_datetime, format_description};"] # [doc = " let format = format_description!(\"[year]-[month]-[day] [hour]:[minute]:[second]\");"] # [doc = " assert_eq!("] # [doc = "     UtcDateTime::parse(\"2020-01-02 03:04:05\", &format)?,"] # [doc = "     utc_datetime!(2020-01-02 03:04:05)"] # [doc = " );"] # [doc = " # Ok::<_, time::Error>(())"] # [doc = " ```"] # [inline] pub fn parse (input : & str , description : & (impl Parsable + ? Sized) ,) -> Result < Self , error :: Parse > { description . parse_utc_date_time (input . as_bytes ()) } # [doc = " A helper method to check if the `UtcDateTime` is a valid representation of a leap second."] # [doc = " Leap seconds, when parsed, are represented as the preceding nanosecond. However, leap"] # [doc = " seconds can only occur as the last second of a month UTC."] # [cfg (feature = "parsing")] # [inline] pub (crate) const fn is_valid_leap_second_stand_in (self) -> bool { let dt = self . inner ; dt . hour () == 23 && dt . minute () == 59 && dt . second () == 59 && dt . nanosecond () == 999_999_999 && dt . day () == dt . month () . length (dt . year ()) } }
};
}
