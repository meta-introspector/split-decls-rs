// Generated macro for quoted_pair (function)
macro_rules! Depcrate_parsing_combinator_rfc_rfc2822quoted_pair {
() => {
// Module: crate::parsing::combinator::rfc::rfc2822
// Provides: {"quoted_pair"}
// Dependencies: {}
# [doc = " Consume the `quoted_pair` rule."] # [inline] fn quoted_pair (mut input : & [u8]) -> Option < ParsedItem < '_ , () > > { input = ascii_char :: < b'\\' > (input) ? . into_inner () ; input = text (input) . into_inner () ; Some (ParsedItem (input , ())) }
};
}
