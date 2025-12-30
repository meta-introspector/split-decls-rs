// Generated macro for lex_comment_tag (function)
macro_rules! Depcrate_parser_tests_lexerlex_comment_tag {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_comment_tag"}
// Dependencies: {}
# [test] fn lex_comment_tag () { let inputs = vec ! ["{# #comment# {{}} {%%} #}" , "{# #comment# {{}} {%%} #}" , "{#- #comment# {{}} {%%} #}" , "{# #comment# {{}} {%%} -#}" , "{#- #comment# {{}} {%%} -#}" ,] ; for i in inputs { assert_lex_rule ! (Rule :: comment_tag , i) ; } }
};
}
