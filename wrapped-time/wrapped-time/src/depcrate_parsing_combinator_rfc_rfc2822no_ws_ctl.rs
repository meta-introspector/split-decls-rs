// Generated macro for no_ws_ctl (function)
macro_rules! Depcrate_parsing_combinator_rfc_rfc2822no_ws_ctl {
() => {
// Module: crate::parsing::combinator::rfc::rfc2822
// Provides: {"no_ws_ctl"}
// Dependencies: {}
# [doc = " Consume the `no_ws_ctl` rule."] # [inline] const fn no_ws_ctl (input : & [u8]) -> Option < ParsedItem < '_ , () > > { match input { [1 ..= 8 | 11 ..= 12 | 14 ..= 31 | 127 , rest @ ..] => Some (ParsedItem (rest , ())) , _ => None , } }
};
}
