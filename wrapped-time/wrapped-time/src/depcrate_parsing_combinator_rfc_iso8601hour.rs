// Generated macro for hour (function)
macro_rules! Depcrate_parsing_combinator_rfc_iso8601hour {
() => {
// Module: crate::parsing::combinator::rfc::iso8601
// Provides: {"hour"}
// Dependencies: {}
# [doc = " Parse the hour."] # [inline] pub (crate) fn hour (input : & [u8]) -> Option < ParsedItem < '_ , u8 > > { exactly_n_digits :: < 2 , _ > (input) }
};
}
