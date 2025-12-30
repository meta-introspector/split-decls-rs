// Generated macro for ccontent (function)
macro_rules! Depcrate_parsing_combinator_rfc_rfc2822ccontent {
() => {
// Module: crate::parsing::combinator::rfc::rfc2822
// Provides: {"ccontent"}
// Dependencies: {}
# [doc = " Consume the `ccontent` rule."] # [inline] fn ccontent (input : & [u8]) -> Option < ParsedItem < '_ , () > > { ctext (input) . or_else (| | quoted_pair (input)) . or_else (| | comment (input)) }
};
}
