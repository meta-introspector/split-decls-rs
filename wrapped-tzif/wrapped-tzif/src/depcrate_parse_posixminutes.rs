// Generated macro for minutes (function)
macro_rules! Depcrate_parse_posixminutes {
() => {
// Module: crate::parse::posix
// Provides: {"minutes"}
// Dependencies: {}
# [doc = " Parses an integer as [`Minutes`] and ensures that it falls within `[0, 59]`."] fn minutes < Input > () -> impl Parser < Input , Output = Minutes > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { bounded_natural (0 , 59) . map (Minutes) }
};
}
