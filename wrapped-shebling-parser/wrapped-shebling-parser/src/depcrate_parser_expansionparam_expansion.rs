// Generated macro for param_expansion (function)
macro_rules! Depcrate_parser_expansionparam_expansion {
() => {
// Module: crate::parser::expansion
// Provides: {"param_expansion"}
// Dependencies: {}
fn param_expansion (span : ParseSpan) -> ParseResult < Vec < WordSgmt > > { delimited (tag ("${") , many0 (alt ((map (spanned (single_quoted) , WordSgmt :: SingleQuoted) , into (double_quoted) , map (spanned (extglob) , WordSgmt :: Glob) , dollar_sgmt , map (spanned (map (many1 (alt ((escaped (BRACED_ESCAPABLE) , recognize_string (is_not (& * format ! ("\\{}" , BRACED_ESCAPABLE))) ,))) , | lits | lits . concat () ,)) , WordSgmt :: Lit ,) ,))) , char ('}') ,) (span) }
};
}
