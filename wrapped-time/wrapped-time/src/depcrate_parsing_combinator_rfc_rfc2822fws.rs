// Generated macro for fws (function)
macro_rules! Depcrate_parsing_combinator_rfc_rfc2822fws {
() => {
// Module: crate::parsing::combinator::rfc::rfc2822
// Provides: {"fws"}
// Dependencies: {}
# [doc = " Consume the `fws` rule."] # [inline] pub (crate) fn fws (mut input : & [u8]) -> Option < ParsedItem < '_ , () > > { if let [b'\r' , b'\n' , rest @ ..] = input { one_or_more (wsp) (rest) } else { input = one_or_more (wsp) (input) ? . into_inner () ; while let [b'\r' , b'\n' , rest @ ..] = input { input = one_or_more (wsp) (rest) ? . into_inner () ; } Some (ParsedItem (input , ())) } }
};
}
