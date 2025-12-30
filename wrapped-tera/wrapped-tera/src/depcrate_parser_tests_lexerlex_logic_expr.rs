// Generated macro for lex_logic_expr (function)
macro_rules! Depcrate_parser_tests_lexerlex_logic_expr {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_logic_expr"}
// Dependencies: {}
# [test] fn lex_logic_expr () { let inputs = vec ! ["1.5 + a | round(var=2) > 10 and admin" , "1.5 + a | round(var=2) > a | round or true" , "1 > 0 and 2 < 3" ,] ; for i in inputs { assert_lex_rule ! (Rule :: logic_expr , i) ; } }
};
}
