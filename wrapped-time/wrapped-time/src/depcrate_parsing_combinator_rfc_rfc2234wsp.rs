// Generated macro for wsp (function)
macro_rules! Depcrate_parsing_combinator_rfc_rfc2234wsp {
() => {
// Module: crate::parsing::combinator::rfc::rfc2234
// Provides: {"wsp"}
// Dependencies: {}
# [doc = " Consume exactly one space or tab."] # [inline] pub (crate) const fn wsp (input : & [u8]) -> Option < ParsedItem < '_ , () > > { match input { [b' ' | b'\t' , rest @ ..] => Some (ParsedItem (rest , ())) , _ => None , } }
};
}
