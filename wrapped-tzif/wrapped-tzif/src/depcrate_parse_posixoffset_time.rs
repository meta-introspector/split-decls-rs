// Generated macro for offset_time (function)
macro_rules! Depcrate_parse_posixoffset_time {
() => {
// Module: crate::parse::posix
// Provides: {"offset_time"}
// Dependencies: {}
# [doc = " Parses a time value of the form `\\[+|-\\]hh\\[:mm\\[:ss\\]\\]`."] # [doc = ""] # [doc = " This is positive if the local time zone is west of the Prime Meridian and negative if it is east."] # [doc = " The hour must be in range `[0, 24]`, and the minute and seconds must be in range `[0, 59]`."] # [doc = ""] # [doc = " Returns the total time in [`Seconds`]."] fn offset_time < Input > () -> impl Parser < Input , Output = Seconds > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { time (24) }
};
}
