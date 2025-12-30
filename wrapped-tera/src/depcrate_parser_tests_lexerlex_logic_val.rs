// Generated macro for lex_logic_val (function)
macro_rules! Depcrate_parser_tests_lexerlex_logic_val {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_logic_val"}
// Dependencies: {}
# [test] fn lex_logic_val () { let inputs = vec ! ["admin" , "true" , "macros::something()" , "something()" , r#""hey""# , "a is defined" , "a is defined(2)" , "a is not defined" , "1 + 1" , "1 + counts" , "1 + counts.first" , "1 + 2 + 3 * 9/2 + 2.1" , "(1 + 2 + 3) * 9/2 + 2.1" , "10 * 2 % 5" , "admin | upper" , "admin | upper | round" , "admin | upper | round(var=2)" , "1.5 + a | round(var=2)" , "not true" , "not admin" , "not num + 1 == 0" ,] ; for i in inputs { assert_lex_rule ! (Rule :: logic_val , i) ; } }
};
}
