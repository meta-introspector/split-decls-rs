// Generated macro for dayo (function)
macro_rules! Depcrate_parsing_combinator_rfc_iso8601dayo {
() => {
// Module: crate::parsing::combinator::rfc::iso8601
// Provides: {"dayo"}
// Dependencies: {}
# [doc = " Parse a day of the year."] # [inline] pub (crate) fn dayo (input : & [u8]) -> Option < ParsedItem < '_ , NonZero < u16 > > > { exactly_n_digits :: < 3 , _ > (input) }
};
}
