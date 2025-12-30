// Generated macro for sign (function)
macro_rules! Depcrate_parse_posixsign {
() => {
// Module: crate::parse::posix
// Provides: {"sign"}
// Dependencies: {}
# [doc = " Parses a plus or minus sign."] fn sign < Input > () -> impl Parser < Input , Output = u8 > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { byte (b'+') . or (byte (b'-')) }
};
}
