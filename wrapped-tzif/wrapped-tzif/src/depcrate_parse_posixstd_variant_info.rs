// Generated macro for std_variant_info (function)
macro_rules! Depcrate_parse_posixstd_variant_info {
() => {
// Module: crate::parse::posix
// Provides: {"std_variant_info"}
// Dependencies: {}
# [doc = " Parses the STD time-zone variant info including the variant name and the offset in seconds."] # [doc = ""] # [doc = " See [`zone_variant_name`] and [`offset_time`] for more information."] fn std_variant_info < Input > () -> impl Parser < Input , Output = TimeZoneVariantInfo > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { combine :: struct_parser ! { TimeZoneVariantInfo { name : zone_variant_name () , offset : offset_time () , } } }
};
}
