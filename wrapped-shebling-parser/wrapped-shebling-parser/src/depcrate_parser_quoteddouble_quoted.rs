// Generated macro for double_quoted (function)
macro_rules! Depcrate_parser_quoteddouble_quoted {
() => {
// Module: crate::parser::quoted
// Provides: {"double_quoted"}
// Dependencies: {}
pub (super) fn double_quoted (span : ParseSpan) -> ParseResult < DoubleQuoted > { fn lit (span : ParseSpan) -> ParseResult < String > { alt ((map (many1 (alt ((escaped (DOUBLE_ESCAPABLE) , recognize_string (is_not (DOUBLE_ESCAPABLE)) ,))) , | sgmts | sgmts . concat () ,) , recognize_string (char ('$')) ,)) (span) } delimited (char ('"') , map (many0 (alt ((into (dollar_exp) , into (spanned (lit)) ,))) , DoubleQuoted :: new ,) , context ("expected ending double quote!" , char ('"')) ,) (span) }
};
}
