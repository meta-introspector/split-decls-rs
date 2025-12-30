// Generated macro for mm_segment (function)
macro_rules! Depcrate_parse_posixmm_segment {
() => {
// Module: crate::parse::posix
// Provides: {"mm_segment"}
// Dependencies: {}
# [doc = " Parses a minutes segment (`:mm`) as part of an `hh:mm:ss` time stamp."] # [doc = " The parsed [`Minutes`] must be within range `[0, 59]`."] # [doc = ""] # [doc = " If there is no leading colon, defaults to zero minutes."] fn mm_segment < Input > () -> impl Parser < Input , Output = Minutes > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { optional (byte (b':')) . then (| colon | { if colon . is_some () { minutes () . left () } else { value (Minutes (0)) . right () } }) }
};
}
