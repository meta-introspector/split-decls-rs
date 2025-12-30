// Generated macro for brace_expansion (function)
macro_rules! Depcrate_parser_expansionbrace_expansion {
() => {
// Module: crate::parser::expansion
// Provides: {"brace_expansion"}
// Dependencies: {}
pub (super) fn brace_expansion (span : ParseSpan) -> ParseResult < Vec < Word > > { fn braced (span : ParseSpan) -> ParseResult < Vec < Word > > { delimited (char ('{') , context ("invalid sequence expression!" , verify (separated_list1 (char (',') , braced_word) , | words : & Vec < Word > | { if words . len () == 1 { let word = & words [0] ; if word . sgmts () . is_empty () { return false ; } else if let Some (lit) = word . as_lit () { return lit . inner () . split_once ("..") . map_or (false , | (prefix , suffix) | { ! prefix . is_empty () && ! prefix . contains ("..") && ! suffix . is_empty () && ! suffix . contains ("..") } ,) ; } } true } ,) ,) , char ('}') ,) (span) } fn braced_word (span : ParseSpan) -> ParseResult < Word > { map (many0 (alt ((map (braced , WordSgmt :: BraceExpansion) , into (dollar_exp) , map (spanned (single_quoted) , WordSgmt :: SingleQuoted) , into (double_quoted) , map (spanned (map (many1 (alt ((escaped ("") , recognize_string (is_not (& * format ! ("{{}}\"$', \t\r\n{}" , trivia :: UNISPACES))) ,))) , | lits | lits . concat () ,)) , WordSgmt :: Lit ,) ,))) , Word :: new ,) (span) } braced (span) }
};
}
