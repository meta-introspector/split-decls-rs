// Generated macro for test_lex_comment (function)
macro_rules! Depcrate_lexer_testtest_lex_comment {
() => {
// Module: crate::lexer::test
// Provides: {"test_lex_comment"}
// Dependencies: {}
# [test] fn test_lex_comment () { let cases = [("#" , str ! [[r#"
Token {
    kind: Comment,
    span: 0..1,
}

"#]] . raw () , str ! [""] . raw () ,) , ("# content" , str ! [[r#"
Token {
    kind: Comment,
    span: 0..9,
}

"#]] . raw () , str ! [""] . raw () ,) , ("# content \ntrailing" , str ! [[r#"
Token {
    kind: Comment,
    span: 0..10,
}

"#]] . raw () , str ! [[r#"

trailing
"#]] . raw () ,) , ("# content \r\ntrailing" , str ! [[r#"
Token {
    kind: Comment,
    span: 0..10,
}

"#]] . raw () , str ! [[r#"

trailing
"#]] . raw () ,) , ("# content \0continue" , str ! [[r#"
Token {
    kind: Comment,
    span: 0..19,
}

"#]] . raw () , str ! [""] . raw () ,) ,] ; for (stream , expected_tokens , expected_stream) in cases { dbg ! (stream) ; let mut stream = Stream :: new (stream) ; let actual_tokens = lex_comment (& mut stream) ; assert_data_eq ! (actual_tokens . to_debug () , expected_tokens . raw ()) ; let stream = * stream ; assert_data_eq ! (stream , expected_stream . raw ()) ; } }
};
}
