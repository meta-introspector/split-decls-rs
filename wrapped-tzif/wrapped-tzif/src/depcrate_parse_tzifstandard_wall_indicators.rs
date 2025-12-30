// Generated macro for standard_wall_indicators (function)
macro_rules! Depcrate_parse_tzifstandard_wall_indicators {
() => {
// Module: crate::parse::tzif
// Provides: {"standard_wall_indicators"}
// Dependencies: {}
# [doc = " A series of standard/wall indicators."] # [doc = " The number of values is specified by the \"isstdcnt\" field in the"] # [doc = " header. If \"isstdcnt\" is zero (0), all transition times"] # [doc = " associated with local time types are assumed to be specified as"] # [doc = " wall time."] fn standard_wall_indicators < Input > (isstdcnt : usize ,) -> impl Parser < Input , Output = Vec < StandardWallIndicator > > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { count_min_max (isstdcnt , isstdcnt , standard_wall_indicator ()) }
};
}
