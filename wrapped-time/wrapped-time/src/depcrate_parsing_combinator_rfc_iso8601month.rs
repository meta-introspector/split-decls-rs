// Generated macro for month (function)
macro_rules! Depcrate_parsing_combinator_rfc_iso8601month {
() => {
// Module: crate::parsing::combinator::rfc::iso8601
// Provides: {"month"}
// Dependencies: {}
# [doc = " Parse a month."] # [inline] pub (crate) fn month (input : & [u8]) -> Option < ParsedItem < '_ , Month > > { first_match ([(b"01" . as_slice () , Month :: January) , (b"02" . as_slice () , Month :: February) , (b"03" . as_slice () , Month :: March) , (b"04" . as_slice () , Month :: April) , (b"05" . as_slice () , Month :: May) , (b"06" . as_slice () , Month :: June) , (b"07" . as_slice () , Month :: July) , (b"08" . as_slice () , Month :: August) , (b"09" . as_slice () , Month :: September) , (b"10" . as_slice () , Month :: October) , (b"11" . as_slice () , Month :: November) , (b"12" . as_slice () , Month :: December) ,] , true ,) (input) }
};
}
