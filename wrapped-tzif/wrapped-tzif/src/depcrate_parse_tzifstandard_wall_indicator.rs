// Generated macro for standard_wall_indicator (function)
macro_rules! Depcrate_parse_tzifstandard_wall_indicator {
() => {
// Module: crate::parse::tzif
// Provides: {"standard_wall_indicator"}
// Dependencies: {}
# [doc = " A one-byte value indicating whether the"] # [doc = " transition times associated with local time types were"] # [doc = " specified as standard time or wall-clock time. Each value MUST be"] # [doc = " 0 or 1. A value of one (1) indicates standard time. The value"] # [doc = " MUST be set to one (1) if the corresponding UT/local indicator is"] # [doc = " set to one (1). A value of zero (0) indicates wall time."] fn standard_wall_indicator < Input > () -> impl Parser < Input , Output = StandardWallIndicator > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { boolean () . map (| bool | { if bool { StandardWallIndicator :: Standard } else { StandardWallIndicator :: Wall } }) }
};
}
