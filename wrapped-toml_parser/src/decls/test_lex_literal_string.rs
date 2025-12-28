macro_rules! deps {
    () => {
        Token!();
        Stream!();
    };
}

macro_rules! test_lex_literal_string {
    () => {
        deps!();
        # [test] fn test_lex_literal_string () { let cases = [("''" , str ! [[r#"
Token {
    kind: LiteralString,
    span: 0..2,
}

"#]] . raw () , str ! [""] . raw () ,) , ("''trailing" , str ! [[r#"
Token {
    kind: LiteralString,
    span: 0..2,
}

"#]] . raw () , str ! ["trailing"] . raw () ,) , ("'content'trailing" , str ! [[r#"
Token {
    kind: LiteralString,
    span: 0..9,
}

"#]] . raw () , str ! ["trailing"] . raw () ,) , ("'content" , str ! [[r#"
Token {
    kind: LiteralString,
    span: 0..8,
}

"#]] . raw () , str ! [""] . raw () ,) , ("'content\ntrailing" , str ! [[r#"
Token {
    kind: LiteralString,
    span: 0..8,
}

"#]] . raw () , str ! [[r#"

trailing
"#]] . raw () ,) ,] ; for (stream , expected_tokens , expected_stream) in cases { dbg ! (stream) ; let mut stream = Stream :: new (stream) ; let actual_tokens = lex_literal_string (& mut stream) ; assert_data_eq ! (actual_tokens . to_debug () , expected_tokens . raw ()) ; let stream = * stream ; assert_data_eq ! (stream , expected_stream . raw ()) ; } }
    };
}

test_lex_literal_string!()