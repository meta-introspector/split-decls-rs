// Generated macro for parse_ok (function)
macro_rules! Depcrate_parser_testsparse_ok {
() => {
// Module: crate::parser::tests
// Provides: {"parse_ok"}
// Dependencies: {}
pub (super) fn parse_ok < P , R > (parser : P , source : & str , remaining : & str ,) -> (Vec < (String , miette :: SourceSpan) > , R) where P : Fn (ParseSpan) -> ParseResult < R > , { let diags = ParseDiags :: new () ; let (span , res) = parser (ParseSpan :: new (source , & diags)) . finish () . expect ("Parsing should succeed") ; assert_str_eq ! (span . fragment () , remaining) ; (summarize_diags (diags) , res) }
};
}
