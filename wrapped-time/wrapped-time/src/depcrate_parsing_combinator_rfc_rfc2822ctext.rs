// Generated macro for ctext (function)
macro_rules! Depcrate_parsing_combinator_rfc_rfc2822ctext {
() => {
// Module: crate::parsing::combinator::rfc::rfc2822
// Provides: {"ctext"}
// Dependencies: {}
# [doc = " Consume the `ctext` rule."] # [expect (clippy :: unnecessary_lazy_evaluations , reason = "rust-lang/rust-clippy#8522")] # [inline] fn ctext (input : & [u8]) -> Option < ParsedItem < '_ , () > > { no_ws_ctl (input) . or_else (| | match input { [33 ..= 39 | 42 ..= 91 | 93 ..= 126 , rest @ ..] => Some (ParsedItem (rest , ())) , _ => None , }) }
};
}
