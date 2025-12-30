// Generated macro for digits (function)
macro_rules! Depcrate_parse_posixdigits {
() => {
// Module: crate::parse::posix
// Provides: {"digits"}
// Dependencies: {}
# [doc = " Parses one ore more digits."] fn digits < Input > () -> impl Parser < Input , Output = Vec < u8 > > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { many1 (digit ()) }
};
}
