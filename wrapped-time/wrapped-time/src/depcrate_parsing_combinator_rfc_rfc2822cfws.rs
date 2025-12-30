// Generated macro for cfws (function)
macro_rules! Depcrate_parsing_combinator_rfc_rfc2822cfws {
() => {
// Module: crate::parsing::combinator::rfc::rfc2822
// Provides: {"cfws"}
// Dependencies: {}
# [doc = " Consume the `cfws` rule."] # [inline] pub (crate) fn cfws (input : & [u8]) -> Option < ParsedItem < '_ , () > > { one_or_more (| input | fws (input) . or_else (| | comment (input))) (input) }
};
}
