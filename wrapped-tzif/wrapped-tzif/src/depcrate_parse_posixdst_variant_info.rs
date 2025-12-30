// Generated macro for dst_variant_info (function)
macro_rules! Depcrate_parse_posixdst_variant_info {
() => {
// Module: crate::parse::posix
// Provides: {"dst_variant_info"}
// Dependencies: {}
# [doc = " Parses the DST time-zone variant info including the variant name and the offset in seconds."] # [doc = ""] # [doc = " This differs from [`std_variant_info`] in that it takes a predetermined STD offset"] # [doc = " offset as an argument. If no explicit DST offset is parsed, it will default to the"] # [doc = " STD offset minus one hour."] fn dst_variant_info < Input > (std_offset : Seconds) -> impl Parser < Input , Output = TimeZoneVariantInfo > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { combine :: struct_parser ! { TimeZoneVariantInfo { name : zone_variant_name () , offset : optional (offset_time ()) . map (move | time | time . unwrap_or (std_offset - Hours (1) . as_seconds ())) , } } }
};
}
