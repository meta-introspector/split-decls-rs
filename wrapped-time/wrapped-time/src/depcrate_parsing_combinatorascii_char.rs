// Generated macro for ascii_char (function)
macro_rules! Depcrate_parsing_combinatorascii_char {
() => {
// Module: crate::parsing::combinator
// Provides: {"ascii_char"}
// Dependencies: {}
# [doc = " Consume exactly one of the provided ASCII characters."] # [inline] pub (crate) fn ascii_char < const CHAR : u8 > (input : & [u8]) -> Option < ParsedItem < '_ , () > > { debug_assert ! (CHAR . is_ascii_graphic () || CHAR . is_ascii_whitespace ()) ; match input { [c , remaining @ ..] if * c == CHAR => Some (ParsedItem (remaining , ())) , _ => None , } }
};
}
