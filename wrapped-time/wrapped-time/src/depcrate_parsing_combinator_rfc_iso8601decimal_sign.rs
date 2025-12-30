// Generated macro for decimal_sign (function)
macro_rules! Depcrate_parsing_combinator_rfc_iso8601decimal_sign {
() => {
// Module: crate::parsing::combinator::rfc::iso8601
// Provides: {"decimal_sign"}
// Dependencies: {}
# [doc = " Parse a \"decimal sign\", which is either a comma or a period."] # [inline] fn decimal_sign (input : & [u8]) -> Option < ParsedItem < '_ , () > > { ascii_char :: < b'.' > (input) . or_else (| | ascii_char :: < b',' > (input)) }
};
}
