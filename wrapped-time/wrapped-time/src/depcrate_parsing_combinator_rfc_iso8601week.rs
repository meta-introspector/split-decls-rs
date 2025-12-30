// Generated macro for week (function)
macro_rules! Depcrate_parsing_combinator_rfc_iso8601week {
() => {
// Module: crate::parsing::combinator::rfc::iso8601
// Provides: {"week"}
// Dependencies: {}
# [doc = " Parse a week number."] # [inline] pub (crate) fn week (input : & [u8]) -> Option < ParsedItem < '_ , NonZero < u8 > > > { exactly_n_digits :: < 2 , _ > (input) }
};
}
