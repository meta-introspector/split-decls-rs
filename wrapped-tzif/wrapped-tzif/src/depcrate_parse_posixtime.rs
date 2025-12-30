// Generated macro for time (function)
macro_rules! Depcrate_parse_posixtime {
() => {
// Module: crate::parse::posix
// Provides: {"time"}
// Dependencies: {}
# [doc = " Parses a time value of the form `\\[+|-\\]hh\\[:mm\\[:ss\\]\\]`."] # [doc = ""] # [doc = " Returns the total time in [`Seconds`]."] fn time < Input > (hour_bound : i64) -> impl Parser < Input , Output = Seconds > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { (hours (hour_bound) , mm_segment () , ss_segment ()) . map (| (hours , minutes , seconds) | { if hours < Hours (0) { hours . as_seconds () - minutes . as_seconds () - seconds } else { hours . as_seconds () + minutes . as_seconds () + seconds } }) }
};
}
