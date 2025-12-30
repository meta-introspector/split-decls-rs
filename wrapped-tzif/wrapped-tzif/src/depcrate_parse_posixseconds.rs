// Generated macro for seconds (function)
macro_rules! Depcrate_parse_posixseconds {
() => {
// Module: crate::parse::posix
// Provides: {"seconds"}
// Dependencies: {}
# [doc = " Parses an integer as [`Seconds`] and ensures that it falls within `[0, 59]`."] fn seconds < Input > () -> impl Parser < Input , Output = Seconds > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { bounded_natural (0 , 59) . map (Seconds) }
};
}
