// Generated macro for posix_tz_string (function)
macro_rules! Depcrate_parse_posixposix_tz_string {
() => {
// Module: crate::parse::posix
// Provides: {"posix_tz_string"}
// Dependencies: {}
# [doc = " Parses a POSIX time-zone string according to the following specification:"] # [doc = " <https://www.gnu.org/software/libc/manual/html_node/TZ-Variable.html>"] # [must_use] pub fn posix_tz_string < Input > () -> impl Parser < Input , Output = PosixTzString > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { std_variant_info () . then (| std_info | { let std_offset = std_info . offset ; combine :: struct_parser ! { PosixTzString { std_info : value (std_info) , dst_info : optional (dst_transition_info (std_offset)) , } } }) }
};
}
