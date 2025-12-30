// Generated macro for lex_include_tag (function)
macro_rules! Depcrate_parser_tests_lexerlex_include_tag {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_include_tag"}
// Dependencies: {}
# [test] fn lex_include_tag () { assert ! (TeraParser :: parse (Rule :: include_tag , "{% include \"index.html\" %}") . is_ok ()) ; assert ! (TeraParser :: parse (Rule :: include_tag , "{% include [\"index.html\"] %}") . is_ok ()) ; assert ! (TeraParser :: parse (Rule :: include_tag , "{% include [\"index.html\"] ignore missing %}") . is_ok ()) ; }
};
}
