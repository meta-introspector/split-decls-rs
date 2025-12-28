macro_rules! deps {
    () => {
        Token!();
        Stream!();
        TokenKind!();
    };
}

macro_rules! test_lex_ascii_char {
    () => {
        deps!();
        # [test] fn test_lex_ascii_char () { let cases = [(".trailing" , str ! [[r#"
Token {
    kind: Dot,
    span: 0..1,
}

"#]] . raw () , str ! ["trailing"] . raw () ,)] ; for (stream , expected_tokens , expected_stream) in cases { dbg ! (stream) ; let mut stream = Stream :: new (stream) ; let actual_tokens = lex_ascii_char (& mut stream , TokenKind :: Dot) ; assert_data_eq ! (actual_tokens . to_debug () , expected_tokens . raw ()) ; let stream = * stream ; assert_data_eq ! (stream , expected_stream . raw ()) ; } }
    };
}

test_lex_ascii_char!()