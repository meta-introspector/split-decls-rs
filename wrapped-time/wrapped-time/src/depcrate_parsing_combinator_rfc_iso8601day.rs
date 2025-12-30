// Generated macro for day (function)
macro_rules! Depcrate_parsing_combinator_rfc_iso8601day {
() => {
// Module: crate::parsing::combinator::rfc::iso8601
// Provides: {"day"}
// Dependencies: {}
# [doc = " Parse a day of the month."] # [inline] pub (crate) fn day (input : & [u8]) -> Option < ParsedItem < '_ , NonZero < u8 > > > { exactly_n_digits :: < 2 , _ > (input) }
};
}
