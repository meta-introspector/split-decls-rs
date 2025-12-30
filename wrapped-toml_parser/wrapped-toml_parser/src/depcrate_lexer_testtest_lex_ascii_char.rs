// Generated macro for test_lex_ascii_char (function)
macro_rules! Depcrate_lexer_testtest_lex_ascii_char {
() => {
// Module: crate::lexer::test
// Provides: {"test_lex_ascii_char"}
// Dependencies: {}
# [test] fn test_lex_ascii_char () { let cases = [(".trailing" , str ! [[r#"
Token {
    kind: Dot,
    span: 0..1,
}

"#]] . raw () , str ! ["trailing"] . raw () ,)] ; for (stream , expected_tokens , expected_stream) in cases { dbg ! (stream) ; let mut stream = Stream :: new (stream) ; let actual_tokens = lex_ascii_char (& mut stream , TokenKind :: Dot) ; assert_data_eq ! (actual_tokens . to_debug () , expected_tokens . raw ()) ; let stream = * stream ; assert_data_eq ! (stream , expected_stream . raw ()) ; } }
};
}
