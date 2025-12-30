// Generated macro for min (function)
macro_rules! Depcrate_parsing_combinator_rfc_iso8601min {
() => {
// Module: crate::parsing::combinator::rfc::iso8601
// Provides: {"min"}
// Dependencies: {}
# [doc = " Parse the minute."] # [inline] pub (crate) fn min (input : & [u8]) -> Option < ParsedItem < '_ , u8 > > { exactly_n_digits :: < 2 , _ > (input) }
};
}
