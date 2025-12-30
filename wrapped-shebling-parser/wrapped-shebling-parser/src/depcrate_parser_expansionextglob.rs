// Generated macro for extglob (function)
macro_rules! Depcrate_parser_expansionextglob {
() => {
// Module: crate::parser::expansion
// Provides: {"extglob"}
// Dependencies: {}
pub (super) fn extglob (span : ParseSpan) -> ParseResult < String > { fn group (span : ParseSpan) -> ParseResult < String > { delimited (char ('(') , sgmt , char (')')) (span) } fn sgmt (span : ParseSpan) -> ParseResult < String > { recognize_string (separated_list0 (char ('|') , recognize_string (many0 (alt ((group , recognize_string (word_sgmt) , recognize_string (many1 (whitespace)) , recognize_string (is_a ("<>#;&")) ,)))) ,)) (span) } recognize_string (pair (one_of (EXTGLOB_PREFIX) , group)) (span) }
};
}
