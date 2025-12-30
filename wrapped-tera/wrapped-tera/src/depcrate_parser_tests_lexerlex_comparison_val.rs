// Generated macro for lex_comparison_val (function)
macro_rules! Depcrate_parser_tests_lexerlex_comparison_val {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_comparison_val"}
// Dependencies: {}
# [test] fn lex_comparison_val () { let inputs = vec ! ["admin" , "true" , "macros::something()" , "something()" , "a is defined" , "a is defined(2)" , "1 + 1" , "1 + counts" , "1 + counts.first" , "1 + 2 + 3 * 9/2 + 2.1" , "(1 + 2 + 3) * 9/2 + 2.1" , "10 * 2 % 5" , "admin | upper" , "admin | upper | round" , "admin | upper | round(var=2)" , "1.5 + a | round(var=2)" , "a | length - 1" , "1.5 + a | round - 1" , "1.5 + a | round - (1 + 1.5) | round" , "1.5 + a | round - (1 + 1.5) | round" ,] ; for i in inputs { assert_lex_rule ! (Rule :: comparison_val , i) ; } }
};
}
