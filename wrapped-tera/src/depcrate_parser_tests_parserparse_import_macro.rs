// Generated macro for parse_import_macro (function)
macro_rules! Depcrate_parser_tests_parserparse_import_macro {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_import_macro"}
// Dependencies: {}
# [test] fn parse_import_macro () { let ast = parse ("\n{% import \"macros.html\" as macros -%}") . unwrap () ; assert_eq ! (ast [0] , Node :: ImportMacro (WS { left : false , right : true } , "macros.html" . to_string () , "macros" . to_string () ,) ,) ; }
};
}
