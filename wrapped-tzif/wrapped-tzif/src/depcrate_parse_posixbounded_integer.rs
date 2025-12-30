// Generated macro for bounded_integer (function)
macro_rules! Depcrate_parse_posixbounded_integer {
() => {
// Module: crate::parse::posix
// Provides: {"bounded_integer"}
// Dependencies: {}
# [doc = " Parses an integer as an i64 and esnures that it falls within `[lower_bound, upper_bound]`."] fn bounded_integer < Input > (lower_bound : i64 , upper_bound : i64) -> impl Parser < Input , Output = i64 > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { assert ! (lower_bound <= upper_bound , "lower bound {lower_bound} was not less than or equal, upper bound {upper_bound}") ; (optional (sign ()) , natural ()) . map (| (sign , natural) | natural * if matches ! (sign , Some (b'-')) { - 1 } else { 1 }) . then (move | integer | { ensure (integer , | & integer | lower_bound <= integer && integer <= upper_bound , "parsed bounded integer is out of bounds" ,) }) }
};
}
