// Generated macro for alphabetic_zone_variant_name_value (function)
macro_rules! Depcrate_parse_posixalphabetic_zone_variant_name_value {
() => {
// Module: crate::parse::posix
// Provides: {"alphabetic_zone_variant_name_value"}
// Dependencies: {}
# [doc = " Parses a byte that not a digit, a comma, a plus nor a minus signs."] fn alphabetic_zone_variant_name_value < Input > () -> impl Parser < Input , Output = u8 > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { satisfy (| byte : u8 | { byte . is_ascii_alphabetic () && ! matches ! (byte , b',' | b'+' | b'-' | b'0' ..= b'9') }) }
};
}
