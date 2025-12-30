// Generated macro for bounded_natural (function)
macro_rules! Depcrate_parse_posixbounded_natural {
() => {
// Module: crate::parse::posix
// Provides: {"bounded_natural"}
// Dependencies: {}
# [doc = " Parses a natural number as an i64 and esnures that it falls within `[lower_bound, upper_bound]`."] fn bounded_natural < Input > (lower_bound : i64 , upper_bound : i64) -> impl Parser < Input , Output = i64 > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { debug_assert ! (lower_bound <= upper_bound , "lower bound {lower_bound} was not less than or equal, upper bound {upper_bound}") ; natural () . then (move | natural | { ensure (natural , | & natural | lower_bound <= natural && natural <= upper_bound , "parsed natural number is out of bounds" ,) }) }
};
}
