// Generated macro for ut_local_indicators (function)
macro_rules! Depcrate_parse_tzifut_local_indicators {
() => {
// Module: crate::parse::tzif
// Provides: {"ut_local_indicators"}
// Dependencies: {}
# [doc = " A series of ut/local indicators"] # [doc = " The number of values is specified by the"] # [doc = " \"isutcnt\" field in the header. If \"isutcnt\" is zero (0), all"] # [doc = " transition times associated with local time types are assumed to"] # [doc = " be specified as local time."] fn ut_local_indicators < Input > (isstdcnt : usize) -> impl Parser < Input , Output = Vec < UtLocalIndicator > > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { count_min_max (isstdcnt , isstdcnt , ut_local_indicator ()) }
};
}
