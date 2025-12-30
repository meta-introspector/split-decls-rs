// Generated macro for parse_fail (function)
macro_rules! Depcrate_parser_testsparse_fail {
() => {
// Module: crate::parser::tests
// Provides: {"parse_fail"}
// Dependencies: {}
pub (super) fn parse_fail < P , R > (parser : P , source : & str ,) -> (ParseError , Vec < (String , miette :: SourceSpan) >) where P : Fn (ParseSpan) -> ParseResult < R > , R : fmt :: Debug , { let diags = ParseDiags :: new () ; let err = parser (ParseSpan :: new (source , & diags)) . finish () . expect_err ("Parsing should fail") ; (err , summarize_diags (diags)) }
};
}
