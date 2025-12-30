// Generated macro for impl_1041 (impl)
macro_rules! Depcrate_timeimpl_1041 {
() => {
// Module: crate::time
// Provides: {"impl_1041"}
// Dependencies: {}
# [cfg (feature = "parsing")] impl Time { # [doc = " Parse a `Time` from the input using the provided [format"] # [doc = " description](crate::format_description)."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time::Time;"] # [doc = " # use time_macros::{time, format_description};"] # [doc = " let format = format_description!(\"[hour]:[minute]:[second]\");"] # [doc = " assert_eq!(Time::parse(\"12:00:00\", &format)?, time!(12:00));"] # [doc = " # Ok::<_, time::Error>(())"] # [doc = " ```"] # [inline] pub fn parse (input : & str , description : & (impl Parsable + ? Sized) ,) -> Result < Self , error :: Parse > { description . parse_time (input . as_bytes ()) } }
};
}
