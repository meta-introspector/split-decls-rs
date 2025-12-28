macro_rules! deps {
    () => {
        Stream!();
        Token!();
    };
}

macro_rules! test_lex_crlf {
    () => {
        deps!();
        # [test] fn test_lex_crlf () { let cases = [("\r\ntrailing" , str ! [[r#"
Token {
    kind: Newline,
    span: 0..2,
}

"#]] . raw () , str ! ["trailing"] . raw () ,) , ("\rtrailing" , str ! [[r#"
Token {
    kind: Newline,
    span: 0..1,
}

"#]] . raw () , str ! ["trailing"] . raw () ,) ,] ; for (stream , expected_tokens , expected_stream) in cases { dbg ! (stream) ; let mut stream = Stream :: new (stream) ; let actual_tokens = lex_crlf (& mut stream) ; assert_data_eq ! (actual_tokens . to_debug () , expected_tokens . raw ()) ; let stream = * stream ; assert_data_eq ! (stream , expected_stream . raw ()) ; } }
    };
}

test_lex_crlf!()