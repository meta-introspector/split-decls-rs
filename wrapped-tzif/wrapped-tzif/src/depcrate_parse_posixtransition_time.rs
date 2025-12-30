// Generated macro for transition_time (function)
macro_rules! Depcrate_parse_posixtransition_time {
() => {
// Module: crate::parse::posix
// Provides: {"transition_time"}
// Dependencies: {}
# [doc = " Parses a time value of the form `\\[+|-\\]hh\\[:mm\\[:ss\\]\\]`."] # [doc = ""] # [doc = " This is positive if the local time zone is west of the Prime Meridian and negative if it is east."] # [doc = " The hour must be in range `[-167, 167]`, and the minute and seconds must be in range `[0, 59]`."] # [doc = " This is an extension to POSIX.1, which only allows hours to be in range `[0, 24]`."] fn transition_time < Input > () -> impl Parser < Input , Output = Seconds > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { optional (byte (b'/')) . then (| slash | { if slash . is_some () { time (167) . left () } else { value (Hours (2) . as_seconds ()) . right () } }) }
};
}
