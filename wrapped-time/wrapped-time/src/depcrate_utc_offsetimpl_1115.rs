// Generated macro for impl_1115 (impl)
macro_rules! Depcrate_utc_offsetimpl_1115 {
() => {
// Module: crate::utc_offset
// Provides: {"impl_1115"}
// Dependencies: {}
# [cfg (feature = "parsing")] impl UtcOffset { # [doc = " Parse a `UtcOffset` from the input using the provided [format"] # [doc = " description](crate::format_description)."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time::UtcOffset;"] # [doc = " # use time_macros::{offset, format_description};"] # [doc = " let format = format_description!(\"[offset_hour]:[offset_minute]\");"] # [doc = " assert_eq!(UtcOffset::parse(\"-03:42\", &format)?, offset!(-3:42));"] # [doc = " # Ok::<_, time::Error>(())"] # [doc = " ```"] # [inline] pub fn parse (input : & str , description : & (impl Parsable + ? Sized) ,) -> Result < Self , error :: Parse > { description . parse_offset (input . as_bytes ()) } }
};
}
