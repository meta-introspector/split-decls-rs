// Generated macro for ss_segment (function)
macro_rules! Depcrate_parse_posixss_segment {
() => {
// Module: crate::parse::posix
// Provides: {"ss_segment"}
// Dependencies: {}
# [doc = " Parses a seconds segment (`:ss`) as part of an `hh:mm:ss` time stamp."] # [doc = " The parsed [`Seconds`] must be within range `[0, 59]`."] # [doc = ""] # [doc = " If there is no leading colon, defaults to zero minutes."] fn ss_segment < Input > () -> impl Parser < Input , Output = Seconds > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { optional (byte (b':')) . then (| colon | { if colon . is_some () { seconds () . left () } else { value (Seconds (0)) . right () } }) }
};
}
