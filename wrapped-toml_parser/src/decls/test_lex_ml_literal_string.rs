macro_rules! deps {
    () => {
        Stream!();
        Token!();
    };
}

macro_rules! test_lex_ml_literal_string {
    () => {
        deps!();
        # [test] fn test_lex_ml_literal_string () { let cases = [("''''''" , str ! [[r#"
Token {
    kind: MlLiteralString,
    span: 0..6,
}

"#]] . raw () , str ! [""] . raw () ,) , ("''''''trailing" , str ! [[r#"
Token {
    kind: MlLiteralString,
    span: 0..6,
}

"#]] . raw () , str ! ["trailing"] . raw () ,) , ("'''content'''trailing" , str ! [[r#"
Token {
    kind: MlLiteralString,
    span: 0..13,
}

"#]] . raw () , str ! ["trailing"] . raw () ,) , ("'''content" , str ! [[r#"
Token {
    kind: MlLiteralString,
    span: 0..10,
}

"#]] . raw () , str ! [""] . raw () ,) , ("'''content'" , str ! [[r#"
Token {
    kind: MlLiteralString,
    span: 0..11,
}

"#]] . raw () , str ! [""] . raw () ,) , ("'''content''" , str ! [[r#"
Token {
    kind: MlLiteralString,
    span: 0..12,
}

"#]] . raw () , str ! [""] . raw () ,) , ("'''content\ntrailing" , str ! [[r#"
Token {
    kind: MlLiteralString,
    span: 0..19,
}

"#]] . raw () , str ! [""] . raw () ,) , ("'''''''trailing" , str ! [[r#"
Token {
    kind: MlLiteralString,
    span: 0..7,
}

"#]] . raw () , str ! ["trailing"] . raw () ,) , ("''''''''trailing" , str ! [[r#"
Token {
    kind: MlLiteralString,
    span: 0..8,
}

"#]] . raw () , str ! ["trailing"] . raw () ,) , ("'''''''''trailing" , str ! [[r#"
Token {
    kind: MlLiteralString,
    span: 0..8,
}

"#]] . raw () , str ! ["'trailing"] . raw () ,) , ("'''''content''''trailing" , str ! [[r#"
Token {
    kind: MlLiteralString,
    span: 0..16,
}

"#]] . raw () , str ! ["trailing"] . raw () ,) , ("'''''content'''''trailing" , str ! [[r#"
Token {
    kind: MlLiteralString,
    span: 0..17,
}

"#]] . raw () , str ! ["trailing"] . raw () ,) , ("'''''content''''''trailing" , str ! [[r#"
Token {
    kind: MlLiteralString,
    span: 0..17,
}

"#]] . raw () , str ! ["'trailing"] . raw () ,) ,] ; for (stream , expected_tokens , expected_stream) in cases { dbg ! (stream) ; let mut stream = Stream :: new (stream) ; let actual_tokens = lex_ml_literal_string (& mut stream) ; assert_data_eq ! (actual_tokens . to_debug () , expected_tokens . raw ()) ; let stream = * stream ; assert_data_eq ! (stream , expected_stream . raw ()) ; } }
    };
}

test_lex_ml_literal_string!();