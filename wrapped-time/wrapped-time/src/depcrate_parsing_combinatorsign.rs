// Generated macro for sign (function)
macro_rules! Depcrate_parsing_combinatorsign {
() => {
// Module: crate::parsing::combinator
// Provides: {"sign"}
// Dependencies: {}
# [doc = " Parse a \"+\" or \"-\" sign. Returns the ASCII byte representing the sign, if present."] # [inline] pub (crate) const fn sign (input : & [u8]) -> Option < ParsedItem < '_ , u8 > > { match input { [sign @ (b'-' | b'+') , remaining @ ..] => Some (ParsedItem (remaining , * sign)) , _ => None , } }
};
}
