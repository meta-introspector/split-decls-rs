// Generated macro for any_digit (function)
macro_rules! Depcrate_parsing_combinatorany_digit {
() => {
// Module: crate::parsing::combinator
// Provides: {"any_digit"}
// Dependencies: {}
# [doc = " Consume exactly one digit."] # [inline] pub (crate) const fn any_digit (input : & [u8]) -> Option < ParsedItem < '_ , u8 > > { match input { [c , remaining @ ..] if c . is_ascii_digit () => Some (ParsedItem (remaining , * c)) , _ => None , } }
};
}
