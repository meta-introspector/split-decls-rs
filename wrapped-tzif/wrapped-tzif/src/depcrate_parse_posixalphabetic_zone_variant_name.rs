// Generated macro for alphabetic_zone_variant_name (function)
macro_rules! Depcrate_parse_posixalphabetic_zone_variant_name {
() => {
// Module: crate::parse::posix
// Provides: {"alphabetic_zone_variant_name"}
// Dependencies: {}
# [doc = " Parses a string that specifies the name of the time zone variant."] # [doc = " It must not contain embedded digits, commas, nor plus and minus signs."] fn alphabetic_zone_variant_name < Input > () -> impl Parser < Input , Output = String > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { many (alphabetic_zone_variant_name_value ()) . map (| bytes : Vec < u8 > | String :: from_utf8_lossy (& bytes) . into_owned ()) }
};
}
