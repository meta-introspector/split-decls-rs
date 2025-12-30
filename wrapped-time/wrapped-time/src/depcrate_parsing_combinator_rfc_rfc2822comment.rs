// Generated macro for comment (function)
macro_rules! Depcrate_parsing_combinator_rfc_rfc2822comment {
() => {
// Module: crate::parsing::combinator::rfc::rfc2822
// Provides: {"comment"}
// Dependencies: {}
# [doc = " Consume the `comment` rule."] # [inline] fn comment (mut input : & [u8]) -> Option < ParsedItem < '_ , () > > { input = ascii_char :: < b'(' > (input) ? . into_inner () ; input = zero_or_more (fws) (input) . into_inner () ; while let Some (rest) = ccontent (input) { input = rest . into_inner () ; input = zero_or_more (fws) (input) . into_inner () ; } input = ascii_char :: < b')' > (input) ? . into_inner () ; Some (ParsedItem (input , ())) }
};
}
