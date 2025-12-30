// Generated macro for ascii_char_ignore_case (function)
macro_rules! Depcrate_parsing_combinatorascii_char_ignore_case {
() => {
// Module: crate::parsing::combinator
// Provides: {"ascii_char_ignore_case"}
// Dependencies: {}
# [doc = " Consume exactly one of the provided ASCII characters, case-insensitive."] # [inline] pub (crate) fn ascii_char_ignore_case < const CHAR : u8 > (input : & [u8]) -> Option < ParsedItem < '_ , () > > { debug_assert ! (CHAR . is_ascii_graphic () || CHAR . is_ascii_whitespace ()) ; match input { [c , remaining @ ..] if c . eq_ignore_ascii_case (& CHAR) => Some (ParsedItem (remaining , ())) , _ => None , } }
};
}
