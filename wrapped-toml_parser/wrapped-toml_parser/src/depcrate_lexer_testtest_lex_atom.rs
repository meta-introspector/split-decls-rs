// Generated macro for test_lex_atom (function)
macro_rules! Depcrate_lexer_testtest_lex_atom {
() => {
// Module: crate::lexer::test
// Provides: {"test_lex_atom"}
// Dependencies: {}
# [test] fn test_lex_atom () { let cases = [("hello" , str ! [[r#"
Token {
    kind: Atom,
    span: 0..5,
}

"#]] . raw () , str ! [""] . raw () ,) , ("hello = world" , str ! [[r#"
Token {
    kind: Atom,
    span: 0..5,
}

"#]] . raw () , str ! [" = world"] . raw () ,) , ("1.100e100 ]" , str ! [[r#"
Token {
    kind: Atom,
    span: 0..1,
}

"#]] . raw () , str ! [".100e100 ]"] . raw () ,) , ("a.b.c = 5" , str ! [[r#"
Token {
    kind: Atom,
    span: 0..1,
}

"#]] . raw () , str ! [".b.c = 5"] . raw () ,) , ("true ]" , str ! [[r#"
Token {
    kind: Atom,
    span: 0..4,
}

"#]] . raw () , str ! [" ]"] . raw () ,) ,] ; for (stream , expected_tokens , expected_stream) in cases { dbg ! (stream) ; let mut stream = Stream :: new (stream) ; let actual_tokens = lex_atom (& mut stream) ; assert_data_eq ! (actual_tokens . to_debug () , expected_tokens . raw ()) ; let stream = * stream ; assert_data_eq ! (stream , expected_stream . raw ()) ; } }
};
}
