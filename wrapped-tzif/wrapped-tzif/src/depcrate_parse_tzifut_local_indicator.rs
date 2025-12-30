// Generated macro for ut_local_indicator (function)
macro_rules! Depcrate_parse_tzifut_local_indicator {
() => {
// Module: crate::parse::tzif
// Provides: {"ut_local_indicator"}
// Dependencies: {}
# [doc = " A one-byte value indicating whether the"] # [doc = " transition times associated with local time types were"] # [doc = " specified as UT or local time. Each value MUST be 0 or 1. A"] # [doc = " value of one (1) indicates UT, and the corresponding standard/wall"] # [doc = " indicator MUST also be set to one (1). A value of zero (0)"] # [doc = " indicates local time."] fn ut_local_indicator < Input > () -> impl Parser < Input , Output = UtLocalIndicator > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { boolean () . map (| bool | { if bool { UtLocalIndicator :: Ut } else { UtLocalIndicator :: Local } }) }
};
}
