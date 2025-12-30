// Generated macro for hours (function)
macro_rules! Depcrate_parse_posixhours {
() => {
// Module: crate::parse::posix
// Provides: {"hours"}
// Dependencies: {}
# [doc = " Parses an integer as [`Hours`] and ensures that it falls within `[-bound, bound]`."] fn hours < Input > (bound : i64) -> impl Parser < Input , Output = Hours > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { bounded_integer (- bound , bound) . map (Hours) }
};
}
