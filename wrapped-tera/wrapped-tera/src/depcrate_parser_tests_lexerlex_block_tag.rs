// Generated macro for lex_block_tag (function)
macro_rules! Depcrate_parser_tests_lexerlex_block_tag {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_block_tag"}
// Dependencies: {}
# [test] fn lex_block_tag () { let inputs = vec ! ["{% block tag %}" , "{% block my_block %}"] ; for i in inputs { assert_lex_rule ! (Rule :: block_tag , i) ; } }
};
}
