// Generated macro for impl_835 (impl)
macro_rules! Depcrate_primitive_date_timeimpl_835 {
() => {
// Module: crate::primitive_date_time
// Provides: {"impl_835"}
// Dependencies: {}
# [cfg (feature = "parsing")] impl PrimitiveDateTime { # [doc = " Parse a `PrimitiveDateTime` from the input using the provided [format"] # [doc = " description](crate::format_description)."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time::PrimitiveDateTime;"] # [doc = " # use time_macros::{datetime, format_description};"] # [doc = " let format = format_description!(\"[year]-[month]-[day] [hour]:[minute]:[second]\");"] # [doc = " assert_eq!("] # [doc = "     PrimitiveDateTime::parse(\"2020-01-02 03:04:05\", &format)?,"] # [doc = "     datetime!(2020-01-02 03:04:05)"] # [doc = " );"] # [doc = " # Ok::<_, time::Error>(())"] # [doc = " ```"] # [inline] pub fn parse (input : & str , description : & (impl Parsable + ? Sized) ,) -> Result < Self , error :: Parse > { description . parse_primitive_date_time (input . as_bytes ()) } }
};
}
