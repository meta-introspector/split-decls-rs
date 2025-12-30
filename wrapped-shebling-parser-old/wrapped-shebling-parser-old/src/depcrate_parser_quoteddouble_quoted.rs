// Generated macro for double_quoted (function)
macro_rules! Depcrate_parser_quoteddouble_quoted {
() => {
// Module: crate::parser::quoted
// Provides: {"double_quoted"}
// Dependencies: {}
pub (super) fn double_quoted (span : Span) -> ParseResult < DoubleQuoted > { delimited (char ('"') , map (many0 (alt ((into (dollar_exp) , into (double_quoted_lit) , into (escaping_backquoted (true)) ,))) , DoubleQuoted :: new ,) , context ("expected ending double quote!" , char ('"')) ,) (span) }
};
}
