// Generated macro for is_dst (function)
macro_rules! Depcrate_parse_tzifis_dst {
() => {
// Module: crate::parse::tzif
// Provides: {"is_dst"}
// Dependencies: {}
# [doc = " A one-byte value indicating whether local time should"] # [doc = " be considered Daylight Saving Time (DST). The value MUST be 0"] # [doc = " or 1. A value of one (1) indicates that this type of time is"] # [doc = " DST. A value of zero (0) indicates that this time type is"] # [doc = " standard time."] fn is_dst < Input > () -> impl Parser < Input , Output = bool > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { boolean () }
};
}
